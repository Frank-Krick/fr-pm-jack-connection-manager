use std::iter;
use std::sync::Arc;
use std::task::Waker;

use anyhow::{Error, Result};
use dashmap::DashMap;
use futures::future::join_all;
use itertools::Itertools;
use pipewire::spa::utils::dict::ParsableValue;

use crate::domain::audio_interface::AudioInterface;
use crate::domain::channel_strip::ChannelStrip;
use crate::domain::looper::Looper;
use crate::domain::lv2_plugin::Lv2Plugin;
use crate::domain::mixer::Mixer;
use crate::domain::port::Port;
use crate::factory::lv2_plugin_future::Lv2PluginFuture;
use crate::fr_pipewire::PipewirePort;
use crate::fr_pm_error::FrPmError;
use crate::registry::pipewire_port_registry::PipewirePortRegistry;
use crate::service::mod_host_service::ModHostService;
use crate::service::pipewire_service::PipewireService;

pub mod lv2_plugin_future;

#[derive(Debug, Clone)]
pub struct FutureState {
    pub waker: Option<Waker>,
}

#[derive(Clone)]
pub struct PerformanceMixerFactory {
    future_states: Arc<DashMap<u16, FutureState>>,
    pipewire_port_registry: PipewirePortRegistry,
    mod_host_service: ModHostService,
    pipewire_service: PipewireService,
}

impl PerformanceMixerFactory {
    pub fn new(
        ports_map: Arc<DashMap<String, PipewirePort>>,
        mod_host_service: ModHostService,
        pipewire_service: PipewireService,
    ) -> PerformanceMixerFactory {
        PerformanceMixerFactory {
            mod_host_service,
            pipewire_service,
            pipewire_port_registry: PipewirePortRegistry { ports_map },
            future_states: Arc::new(DashMap::new()),
        }
    }

    pub fn build_lv2_plugin(&self, uri: String) -> Result<Lv2PluginFuture> {
        let index = self.mod_host_service.add_lv2_plugin(uri)?;
        self.future_states
            .insert(index, FutureState { waker: None });
        Ok(Lv2PluginFuture {
            index,
            pipewire_port_registry: self.pipewire_port_registry.clone(),
            future_states: self.future_states.clone(),
        })
    }

    pub fn data_changed(&self) {
        for future_state in self.future_states.iter() {
            match future_state.value().waker.clone() {
                None => {}
                Some(waker) => waker.wake(),
            }
        }
    }

    pub fn get_sooperlooper(&self) -> Result<Vec<Looper>> {
        let node = if let Some(node) = self
            .pipewire_port_registry
            .get_nodes()
            .into_iter()
            .filter(|n| n.name.contains("sooperlooper"))
            .next()
        {
            node
        } else {
            return Err(Error::from(FrPmError {
                message: String::from("Couldn't find playback device"),
            }));
        };

        let keys = node
            .in_ports
            .iter()
            .filter(|p| p.name.contains("loop"))
            .map(|p| p.name.split(":").next().unwrap().split("_").next().unwrap())
            .unique();

        let loopers = keys
            .map(|k| {
                let (_, index) = k.split_at(4);
                Looper {
                    index: u16::parse_value(index).unwrap(),
                    name: format!("SooperLooper:{index}"),
                    input_ports: node
                        .in_ports
                        .iter()
                        .filter(|p| p.name.contains(k))
                        .map(|p| Port::from(p))
                        .collect(),
                    output_ports: node
                        .out_ports
                        .iter()
                        .filter(|p| p.name.contains(k))
                        .map(|p| Port::from(p))
                        .collect(),
                }
            })
            .sorted_by_key(|l| l.name.clone());

        Ok(loopers.collect())
    }

    pub fn get_rme_raydat_audio_interface(&self) -> Result<AudioInterface> {
        let playback_node = if let Some(playback_node) = self
            .pipewire_port_registry
            .get_nodes()
            .into_iter()
            .filter(|n| n.name.contains("alsa:acp:HDSPMxa5963e:3:playback"))
            .next()
        {
            playback_node
        } else {
            return Err(Error::from(FrPmError {
                message: String::from("Couldn't find playback device"),
            }));
        };

        let capture_node = if let Some(capture_node) = self
            .pipewire_port_registry
            .get_nodes()
            .into_iter()
            .filter(|n| n.name.contains("alsa:acp:HDSPMxa5963e:2"))
            .next()
        {
            capture_node
        } else {
            return Err(Error::from(FrPmError {
                message: String::from("Couldn't find capture device"),
            }));
        };

        Ok(AudioInterface {
            name: String::from("alsa:acp:HDSPMxa5963e"),
            input_ports: playback_node
                .in_ports
                .iter()
                .map(|p| Port::from(p))
                .collect(),
            output_ports: capture_node
                .out_ports
                .iter()
                .map(|p| Port::from(p))
                .collect(),
        })
    }

    pub async fn build_cross_faders(&self) -> Vec<Lv2Plugin> {
        let faders_async = [
            self.build_lv2_plugin(String::from("http://gareus.org/oss/lv2/xfade"))
                .unwrap(),
            self.build_lv2_plugin(String::from("http://gareus.org/oss/lv2/xfade"))
                .unwrap(),
            self.build_lv2_plugin(String::from("http://gareus.org/oss/lv2/xfade"))
                .unwrap(),
            self.build_lv2_plugin(String::from("http://gareus.org/oss/lv2/xfade"))
                .unwrap(),
            self.build_lv2_plugin(String::from("http://gareus.org/oss/lv2/xfade"))
                .unwrap(),
            self.build_lv2_plugin(String::from("http://gareus.org/oss/lv2/xfade"))
                .unwrap(),
            self.build_lv2_plugin(String::from("http://gareus.org/oss/lv2/xfade"))
                .unwrap(),
            self.build_lv2_plugin(String::from("http://gareus.org/oss/lv2/xfade"))
                .unwrap(),
            self.build_lv2_plugin(String::from("http://gareus.org/oss/lv2/xfade"))
                .unwrap(),
            self.build_lv2_plugin(String::from("http://gareus.org/oss/lv2/xfade"))
                .unwrap(),
        ];
        return join_all(faders_async).await;
    }

    pub async fn build_inputs_mixer(&self) -> Mixer {
        let channels_async = [
            self.build_channel_strip(),
            self.build_channel_strip(),
            self.build_channel_strip(),
            self.build_channel_strip(),
            self.build_channel_strip(),
            self.build_channel_strip(),
            self.build_channel_strip(),
            self.build_channel_strip(),
        ];
        let channels = join_all(channels_async).await;
        Mixer::new(&channels)
    }

    pub async fn build_group_mixer(&self) -> Mixer {
        let channels_async = [
            self.build_channel_strip(),
            self.build_channel_strip(),
            self.build_channel_strip(),
            self.build_channel_strip(),
        ];
        let channels = join_all(channels_async).await;
        Mixer::new(&channels)
    }

    pub fn connect_input_mixer_and_group_mixer(&self, input_mixer: &Mixer, group_mixer: &Mixer) {
        let drums_input_ports = &group_mixer.channels[0].saturator.input_ports;
        let bass_input_ports = &group_mixer.channels[1].saturator.input_ports;
        let melody_input_ports = &group_mixer.channels[2].saturator.input_ports;
        let atmos_input_ports = &group_mixer.channels[3].saturator.input_ports;

        connect_matched_audio_ports(
            &input_mixer.channels[0].gain.output_ports,
            melody_input_ports,
            &self.pipewire_service,
        );

        connect_matched_audio_ports(
            &input_mixer.channels[1].gain.output_ports,
            bass_input_ports,
            &self.pipewire_service,
        );

        connect_matched_audio_ports(
            &input_mixer.channels[2].gain.output_ports,
            atmos_input_ports,
            &self.pipewire_service,
        );

        connect_matched_audio_ports(
            &input_mixer.channels[3].gain.output_ports,
            melody_input_ports,
            &self.pipewire_service,
        );

        connect_matched_audio_ports(
            &input_mixer.channels[4].gain.output_ports,
            drums_input_ports,
            &self.pipewire_service,
        );

        connect_matched_audio_ports(
            &input_mixer.channels[5].gain.output_ports,
            melody_input_ports,
            &self.pipewire_service,
        );

        connect_matched_audio_ports(
            &input_mixer.channels[6].gain.output_ports,
            melody_input_ports,
            &self.pipewire_service,
        );

        connect_matched_audio_ports(
            &input_mixer.channels[7].gain.output_ports,
            melody_input_ports,
            &self.pipewire_service,
        );
    }

    pub fn connect_group_mixer_and_audio_interface_inputs(
        &self,
        audio_interface: &AudioInterface,
        groups_mixer: &Mixer,
    ) {
        let sorted_audio_input_ports = audio_interface
            .input_ports
            .iter()
            .filter(|p| p.dsp_format.contains("audio"))
            .sorted_by_key(|p| u32::parse_value(&*p.id))
            .collect::<Vec<&Port>>();

        let stereo_output_ports = [
            sorted_audio_input_ports[0].clone(),
            sorted_audio_input_ports[1].clone(),
        ]
        .to_vec();

        for channel_index in 0..4 {
            connect_matched_audio_ports(
                &groups_mixer.channels[channel_index].gain.output_ports,
                &stereo_output_ports,
                &self.pipewire_service,
            );
        }
    }

    pub fn connect_looper_output_to_fader_input(
        &self,
        loopers: &Vec<Looper>,
        faders: &Vec<Lv2Plugin>,
    ) {
        let sorted_audio_output_ports = loopers
            .iter()
            .map(|l| {
                l.output_ports
                    .iter()
                    .filter(|p| p.dsp_format.contains("audio"))
                    .sorted_by_key(|p| u32::parse_value(&*p.id))
            })
            .flatten()
            .collect();

        let sorted_audio_input_ports = faders
            .iter()
            .map(|f| {
                f.input_ports
                    .iter()
                    .filter(|p| p.dsp_format.contains("audio"))
                    .sorted_by_key(|p| u32::parse_value(&*p.id))
                    .skip(2)
            })
            .flatten()
            .collect();

        self.zip_and_connect(&sorted_audio_output_ports, &sorted_audio_input_ports);
    }

    fn zip_and_connect(&self, output_ports: &Vec<&Port>, input_ports: &Vec<&Port>) {
        let matched_channels = iter::zip(output_ports, input_ports);
        for matched_channel in matched_channels {
            self.pipewire_service.connect(
                matched_channel.0.node_id.clone(),
                matched_channel.0.id.clone(),
                matched_channel.1.node_id.clone(),
                matched_channel.1.id.clone(),
            );
        }
    }

    pub fn connect_fader_and_audio_interface_outputs(
        &self,
        audio_interface: &AudioInterface,
        faders: &Vec<Lv2Plugin>,
    ) {
        let sorted_audio_output_ports = audio_interface
            .output_ports
            .iter()
            .filter(|p| p.dsp_format.contains("audio"))
            .sorted_by_key(|p| u32::parse_value(&*p.id))
            .collect();

        let sorted_audio_input_ports = faders[0..8]
            .iter()
            .map(|f| {
                f.input_ports
                    .iter()
                    .filter(|p| p.dsp_format.contains("audio"))
                    .sorted_by_key(|p| p.id.clone())
                    .take(2)
            })
            .flatten()
            .collect();

        self.zip_and_connect(&sorted_audio_output_ports, &sorted_audio_input_ports);
    }

    pub fn connect_loopers_and_audio_interface_outputs(
        &self,
        audio_interface: &AudioInterface,
        loopers: &Vec<Looper>,
    ) {
        let sorted_audio_output_ports = audio_interface
            .output_ports
            .iter()
            .filter(|p| p.dsp_format.contains("audio"))
            .sorted_by_key(|p| u32::parse_value(&*p.id))
            .collect();

        let sorted_audio_input_ports = loopers[0..8]
            .iter()
            .map(|c| {
                c.input_ports
                    .iter()
                    .filter(|p| p.dsp_format.contains("audio"))
                    .sorted_by_key(|p| u32::parse_value(&*p.id))
                    .clone()
            })
            .flatten()
            .collect();

        self.zip_and_connect(&sorted_audio_output_ports, &sorted_audio_input_ports);
    }

    pub fn connect_input_mixer_and_fader_outputs(
        &self,
        faders: &Vec<Lv2Plugin>,
        inputs_mixer: &Mixer,
    ) {
        let sorted_audio_output_ports = faders.iter()
            .map(|f| f.output_ports.iter()
                .filter(|p| p.dsp_format.contains("audio"))
                .sorted_by_key(|p| u32::parse_value(&*p.id))
            ).flatten().collect();

        let sorted_audio_input_ports = inputs_mixer
            .channels
            .iter()
            .map(|c| {
                c.saturator
                    .input_ports
                    .iter()
                    .filter(|p| p.dsp_format.contains("audio"))
                    .sorted_by_key(|p| u32::parse_value(&*p.id))
                    .clone()
            })
            .flatten()
            .collect();

        self.zip_and_connect(&sorted_audio_output_ports, &sorted_audio_input_ports);
    }

    pub async fn build_channel_strip(&self) -> ChannelStrip {
        let lv2_plugin_saturator = self
            .build_lv2_plugin(String::from(
                "http://calf.sourceforge.net/plugins/Saturator",
            ))
            .unwrap()
            .await;

        let lv2_plugin_compressor = self
            .build_lv2_plugin(String::from(
                "http://calf.sourceforge.net/plugins/Compressor",
            ))
            .unwrap()
            .await;

        connect_matched_audio_ports(
            &lv2_plugin_saturator.output_ports,
            &lv2_plugin_compressor.input_ports,
            &self.pipewire_service,
        );

        let lv2_plugin_eq = self
            .build_lv2_plugin(String::from("http://distrho.sf.net/plugins/3BandEQ"))
            .unwrap()
            .await;

        connect_matched_audio_ports(
            &lv2_plugin_compressor.output_ports,
            &lv2_plugin_eq.input_ports,
            &self.pipewire_service,
        );

        let lv2_plugin_gain = self
            .build_lv2_plugin(String::from(
                "http://kxstudio.sf.net/carla/plugins/audiogain_s",
            ))
            .unwrap()
            .await;

        connect_matched_audio_ports(
            &lv2_plugin_eq.output_ports,
            &lv2_plugin_gain.input_ports,
            &self.pipewire_service,
        );

        ChannelStrip::new(
            lv2_plugin_saturator,
            lv2_plugin_compressor,
            lv2_plugin_eq,
            lv2_plugin_gain,
        )
    }
}

fn connect_matched_audio_ports(
    output_ports: &Vec<Port>,
    input_ports: &Vec<Port>,
    pipewire_service: &PipewireService,
) {
    let sorted_audio_output_ports = output_ports
        .iter()
        .filter(|p| p.dsp_format.contains("audio"))
        .sorted_by_key(|p| u32::parse_value(&*p.id));

    let sorted_audio_input_ports = input_ports
        .iter()
        .filter(|p| p.dsp_format.contains("audio"))
        .sorted_by_key(|p| u32::parse_value(&*p.id));

    let matched_channels = iter::zip(sorted_audio_output_ports, sorted_audio_input_ports);
    for matched_channel in matched_channels {
        pipewire_service.connect(
            matched_channel.0.node_id.clone(),
            matched_channel.0.id.clone(),
            matched_channel.1.node_id.clone(),
            matched_channel.1.id.clone(),
        );
    }
}
