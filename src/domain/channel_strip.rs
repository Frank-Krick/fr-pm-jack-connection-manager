use crate::domain::channel::Channel;
use crate::domain::jack_connection_manager::JackConnectionManager;
use crate::domain::lv2_processor::calv_audio_plugin::CalvAudioPlugin;
use crate::domain::lv2_processor::distrho_3band_eq::Distrho3BandEq;
use crate::domain::lv2_processor::Lv2Processor;

pub struct ChannelStrip {
    distortion: CalvAudioPlugin,
    compressor: CalvAudioPlugin,
    equalizer: Distrho3BandEq,
}

impl ChannelStrip {
    pub fn new(postfix: &str) -> ChannelStrip {
        ChannelStrip {
            distortion: CalvAudioPlugin::new(format!("st_dst_{}", postfix)),
            compressor: CalvAudioPlugin::new(format!("st_cmp_{}", postfix)),
            equalizer: Distrho3BandEq::new(format!("st_eq_{}", postfix))
        }
    }

    pub fn input_channels(&self) -> &[Channel] {
        self.distortion.input_channels()
    }

    pub fn output_channels(&self) -> &[Channel] {
        self.equalizer.output_channels()
    }

    pub fn connect_internals(&self, jack_connection_manager: &JackConnectionManager) {
        jack_connection_manager.connect(
            &self.distortion.output_channels()[0],
            &self.compressor.input_channels()[0]
        );
        jack_connection_manager.connect(
            &self.distortion.output_channels()[1],
            &self.compressor.input_channels()[1]
        );
        jack_connection_manager.connect(
            &self.compressor.output_channels()[0],
            &self.equalizer.input_channels()[0]
        );
        jack_connection_manager.connect(
            &self.compressor.output_channels()[1],
            &self.equalizer.input_channels()[1]
        );
    }
}
