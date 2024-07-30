use anyhow::Result;
use dashmap::DashMap;
use pipewire::channel;
use pipewire::context::Context;
use pipewire::loop_::Signal;
use pipewire::main_loop::MainLoop;
use pipewire::spa::utils::dict::DictRef;
use pipewire::types::ObjectType;
use std::cell::OnceCell;
use std::future::Future;
use std::net::TcpStream;
use std::pin::pin;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Barrier};
use std::task::Poll;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::channel_messages::{ModHostCommand, PipewireCommand};
use crate::client::mod_host_client::ModHostClient;
use crate::domain::lv2_plugin::Lv2Plugin;
use crate::factory::PerformanceMixerFactory;
use crate::fr_pipewire::{PipewireDevice, PipewirePort};
use crate::service::mod_host_service::ModHostService;
use crate::service::pipewire_service::PipewireService;

mod channel_messages;
mod client;
mod domain;
mod factory;
mod fr_pipewire;
mod registry;
mod service;

fn main() -> Result<()> {
    let barrier = Arc::new(Barrier::new(2));

    let devices_map = Arc::new(DashMap::<String, PipewireDevice>::new());
    let ports_map = Arc::new(DashMap::<String, PipewirePort>::new());

    let (pipewire_sender, pipewire_receiver) = channel::channel();

    let (mod_host_queue_sender, mod_host_queue_receiver) = mpsc::channel();
    let _mod_host_service_thread = start_mod_host_service_thread(mod_host_queue_receiver)?;

    let ports_map_clone = ports_map.clone();
    let barrier_clone = barrier.clone();
    let mod_host_service = ModHostService::new(mod_host_queue_sender.clone());
    let pipewire_service = PipewireService::new(pipewire_sender.clone());
    let mut factory =
        PerformanceMixerFactory::new(ports_map_clone, mod_host_service, pipewire_service);
    let mut cloned_factory = factory.clone();
    let _control_thread = thread::spawn(move || {
        barrier_clone.wait();

        let lv2_plugin = cloned_factory
            .add_lv2_plugin(String::from(
                "http://calf.sourceforge.net/plugins/Compressor",
            ))
            .unwrap();

        let mut pinned = pin!(lv2_plugin);
        let waker = futures::task::noop_waker_ref();
        let mut cx = std::task::Context::from_waker(waker);

        while match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(lv2_plugin) => {
                println!("{:?}", lv2_plugin);
                false
            }
            Poll::Pending => {
                thread::sleep(Duration::from_secs(1));
                true
            }
        } {}

    });

    let devices_map_pipewire_loop = devices_map.clone();
    let ports_map_clone = ports_map.clone();
    let barrier_clone = barrier.clone();
    let mut cloned_factory = factory.clone();
    let _ = start_pipewire_loop(
        pipewire_receiver,
        devices_map_pipewire_loop,
        ports_map_clone,
        barrier_clone,
        cloned_factory
    );

    Ok(())
}

fn start_pipewire_loop(
    pipewire_receiver: pipewire::channel::Receiver<PipewireCommand>,
    devices_map: Arc<DashMap<String, PipewireDevice>>,
    ports_map: Arc<DashMap<String, PipewirePort>>,
    barrier: Arc<Barrier>,
    performance_mixer_factory: PerformanceMixerFactory
) -> Result<()> {
    pipewire::init();
    let main_loop = MainLoop::new(None)?;
    let context = Context::new(&main_loop)?;
    let core = context.connect(None)?;
    let registry = core.get_registry()?;

    let factory: Rc<OnceCell<String>> = Rc::new(OnceCell::new());
    let factory_clone = factory.clone();

    let reg_listener_device_map = devices_map.clone();
    let ports_map_clone = ports_map.clone();
    let main_loop_weak = main_loop.downgrade();
    let clone_factory = performance_mixer_factory.clone();
    let reg_listener = registry
        .add_listener_local()
        .global(move |global| {
            if let Some(props) = global.props {
                handle_device_update(reg_listener_device_map.clone(), props);
                handle_port_update(ports_map_clone.clone(), props, &clone_factory);

                if props.get("factory.type.name") == Some(ObjectType::Link.to_str()) {
                    let factory_name = props.get("factory.name").expect("Factory has no name");
                    let _ = factory_clone.set(factory_name.to_owned());
                }

                // We found the factory we needed, so quit the loop.
                if factory_clone.get().is_some() {
                    if let Some(main_loop) = main_loop_weak.upgrade() {
                        main_loop.quit();
                    }
                }
            }
        })
        .register();

    main_loop.run();

    drop(reg_listener);

    let reg_device_and_link_listener_devices_map = devices_map.clone();
    let ports_map_clone = ports_map.clone();
    let clone_factory = performance_mixer_factory.clone();
    let reg_device_and_link_listener = registry
        .add_listener_local()
        .global(move |global| {
            if let Some(props) = global.props {
                handle_device_update(reg_device_and_link_listener_devices_map.clone(), props);
                handle_port_update(ports_map_clone.clone(), props, &clone_factory);
            }
        })
        .register();

    let main_loop_weak = main_loop.downgrade();
    let _sig_int = main_loop.loop_().add_signal_local(Signal::SIGINT, move || {
        if let Some(main_loop) = main_loop_weak.upgrade() {
            main_loop.quit();
        }
    });

    let main_loop_weak = main_loop.downgrade();
    let _sig_term = main_loop
        .loop_()
        .add_signal_local(Signal::SIGTERM, move || {
            if let Some(main_loop) = main_loop_weak.upgrade() {
                main_loop.quit();
            }
        });

    let _receiver = pipewire_receiver.attach(main_loop.loop_(), move |command: PipewireCommand| {
        match command {
            PipewireCommand::Connect => {
                core.create_object::<pipewire::link::Link>(
                    factory.get().unwrap(),
                    &pipewire::properties::properties! {
                            "link.output.port" => "0",
                            "link.input.port" => "0",
                            "link.output.node" => "73",
                            "link.input.node" => "72",
                    // Don't remove the object on the remote when we destroy our proxy.
                            "object.linger" => "1"
                        },
                )
                .unwrap();
            }
        }
    });

    barrier.wait();

    main_loop.run();

    Ok(())
}

fn handle_device_update(devices_map: Arc<DashMap<String, PipewireDevice>>, props: &DictRef) {
    match props.get("device.name") {
        None => {}
        Some(value) => {
            let value_copy = String::from(value);
            devices_map.insert(
                value_copy.clone(),
                PipewireDevice {
                    name: value_copy.clone(),
                    factory_id: String::from(props.get("factory.id").unwrap()),
                    client_id: String::from(props.get("client.id").unwrap()),
                    description: String::from(props.get("device.description").unwrap()),
                    nick: String::from(props.get("device.nick").unwrap()),
                    media_class: String::from(props.get("media.class").unwrap()),
                    object_serial: String::from(props.get("object.serial").unwrap()),
                },
            );
        }
    }
}

fn handle_port_update(ports_map: Arc<DashMap<String, PipewirePort>>,
                      props: &DictRef,
                      performance_mixer_factory: &PerformanceMixerFactory) {
    match props.get("port.name") {
        None => {}
        Some(_value) => {
            let port = PipewirePort {
                id: String::from(props.get("port.id").unwrap()),
                name: String::from(props.get("port.name").unwrap()),
                direction: String::from(props.get("port.direction").unwrap()),
                physical: String::from(props.get("port.physical").unwrap_or_else(|| "")),
                alias: String::from(props.get("port.alias").unwrap()),
                group: String::from(props.get("port.group").unwrap_or_else(|| "")),
                path: String::from(props.get("object.path").unwrap()),
                dsp_format: String::from(props.get("format.dsp").unwrap()),
                node_id: String::from(props.get("node.id").unwrap()),
                audio_channel: String::from(props.get("audio.channel").unwrap_or_else(|| "")),
            };

            ports_map.insert(port.alias.clone(), port);
            performance_mixer_factory.data_changed();
        }
    }
}

fn start_mod_host_service_thread(
    mod_host_queue_receiver: Receiver<ModHostCommand>,
) -> Result<JoinHandle<()>> {
    let stream = TcpStream::connect("127.0.0.1:5555")?;
    let mod_host_service_thread = thread::spawn(move || {
        let mut mod_host_client = ModHostClient::new(stream);
        loop {
            match mod_host_queue_receiver.recv() {
                Ok(command) => match command {
                    ModHostCommand::Add(lv2_plugin_uri, index) => {
                        match mod_host_client.add_plugin(lv2_plugin_uri.as_str(), index) {
                            Ok(_) => {
                                println!("Created")
                            }
                            Err(error) => {
                                print!("Error {}", error)
                            }
                        }
                    }
                },
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
    });
    Ok(mod_host_service_thread)
}
