pub mod lv2_plugin_future;

use std::iter;
use anyhow::Result;

use crate::fr_pipewire::PipewirePort;
use crate::registry::pipewire_port_registry::PipewirePortRegistry;
use crate::service::mod_host_service::ModHostService;
use dashmap::DashMap;
use std::sync::Arc;
use std::task::Waker;
use itertools::Itertools;
use pipewire::spa::utils::dict::ParsableValue;
use crate::domain::lv2_plugin::Lv2Plugin;
use crate::domain::port::Port;
use crate::factory::lv2_plugin_future::Lv2PluginFuture;
use crate::service::pipewire_service::PipewireService;

#[derive(Debug, Clone)]
pub struct FutureState {
    pub waker: Option<Waker>,
}

#[derive(Clone)]
pub struct PerformanceMixerFactory {
    future_states: Arc<DashMap<u16, FutureState>>,
    pipewire_port_registry: PipewirePortRegistry,
    mod_host_service: ModHostService,
    pipewire_service: PipewireService
}

impl PerformanceMixerFactory {
    pub fn new(
        ports_map: Arc<DashMap<String, PipewirePort>>,
        mod_host_service: ModHostService,
        pipewire_service: PipewireService
    ) -> PerformanceMixerFactory {
        PerformanceMixerFactory {
            mod_host_service,
            pipewire_service,
            pipewire_port_registry: PipewirePortRegistry { ports_map },
            future_states: Arc::new(DashMap::new())
        }
    }

    pub fn add_lv2_plugin(&mut self, uri: String) -> Result<Lv2PluginFuture> {
        let index = self.mod_host_service.add_lv2_plugin(uri)?;
        self.future_states.insert(index, FutureState { waker: None });
        Ok(Lv2PluginFuture {
            index,
            pipewire_port_registry: self.pipewire_port_registry.clone(),
            future_states: self.future_states.clone()
        })
    }

    pub fn data_changed(&self) {
        for future_state in self.future_states.iter() {
            match future_state.value().waker.clone() {
                None => {}
                Some(waker) => { waker.wake() }
            }
        }
    }

    pub async fn build_channel_strip(&mut self) -> [Lv2Plugin; 4] {
        let lv2_plugin_saturator = self
            .add_lv2_plugin(String::from(
                "http://calf.sourceforge.net/plugins/Saturator",
            ))
            .unwrap()
            .await;

        let lv2_plugin_compressor = self
            .add_lv2_plugin(String::from(
                "http://calf.sourceforge.net/plugins/Compressor",
            ))
            .unwrap()
            .await;

        connect_matched_audio_ports(
            &lv2_plugin_saturator.output_ports,
            &lv2_plugin_compressor.input_ports,
            &self.pipewire_service);

        let lv2_plugin_eq = self
            .add_lv2_plugin(String::from("http://distrho.sf.net/plugins/3BandEQ"))
            .unwrap()
            .await;

        connect_matched_audio_ports(
            &lv2_plugin_compressor.output_ports,
            &lv2_plugin_eq.input_ports,
            &self.pipewire_service);

        let lv2_plugin_gain = self
            .add_lv2_plugin(String::from("http://kxstudio.sf.net/carla/plugins/audiogain_s"))
            .unwrap()
            .await;

        connect_matched_audio_ports(
            &lv2_plugin_eq.output_ports,
            &lv2_plugin_gain.input_ports,
            &self.pipewire_service);

        [lv2_plugin_saturator, lv2_plugin_compressor, lv2_plugin_eq, lv2_plugin_gain]
    }
}

fn connect_matched_audio_ports(output_ports: &Vec<Port>,
                               input_ports: &Vec<Port>,
                               pipewire_service: &PipewireService) {
    let saturator_output_channels = output_ports.iter().filter(|p| {
        p.dsp_format.contains("audio")
    }).sorted_by_key(|p| u32::parse_value(&*p.id));

    let compressor_input_channels = input_ports.iter().filter(|p| {
        p.dsp_format.contains("audio")
    }).sorted_by_key(|p| u32::parse_value(&*p.id));

    let matched_channels = iter::zip(saturator_output_channels, compressor_input_channels);
    for matched_channel in matched_channels {
        pipewire_service.connect(
            matched_channel.0.node_id.clone(),
            matched_channel.0.id.clone(),
            matched_channel.1.node_id.clone(),
            matched_channel.1. id.clone()
        );
    }
}
