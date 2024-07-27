use crate::domain::channel::Channel;
use crate::domain::channel_strip::ChannelStrip;
use crate::domain::jack_connection_manager::JackConnectionManager;
use crate::domain::jack_mixer::drum_mixer::DrumMixer;
use crate::domain::jack_mixer::JackMixer;
use crate::domain::lv2_generator::Lv2Generator;
use crate::domain::lv2_generator::sfizz_multi::SFizzMulti;

pub struct DigitalDrumSection {
    channel_strips: [ChannelStrip; 8],
    sample_player: SFizzMulti,
    mixer: DrumMixer
}

impl DigitalDrumSection {
    pub fn new() -> DigitalDrumSection {
        DigitalDrumSection {
            channel_strips: [
                ChannelStrip::new("_drm_1"),
                ChannelStrip::new("_drm_2"),
                ChannelStrip::new("_drm_3"),
                ChannelStrip::new("_drm_4"),
                ChannelStrip::new("_drm_5"),
                ChannelStrip::new("_drm_6"),
                ChannelStrip::new("_drm_7"),
                ChannelStrip::new("_drm_8"),
            ],
            sample_player: SFizzMulti::new(),
            mixer: DrumMixer::new()
        }
    }

    pub fn connect_internals(&self, jack_connection_manager: &JackConnectionManager) {
        for channel_index in 0..8 {
            self.channel_strips[channel_index].connect_internals(jack_connection_manager);
            jack_connection_manager.connect(
                &self.sample_player.output_channels()[2 * channel_index],
                &self.channel_strips[channel_index].input_channels()[0]
            );
            jack_connection_manager.connect(
                &self.sample_player.output_channels()[2 * channel_index + 1],
                &self.channel_strips[channel_index].input_channels()[1]
            );
            jack_connection_manager.connect(
                &self.channel_strips[channel_index].output_channels()[0],
                &self.mixer.output_channels()[2 * channel_index]
            );
            jack_connection_manager.connect(
                &self.channel_strips[channel_index].output_channels()[1],
                &self.mixer.output_channels()[2 * channel_index + 1]
            );
        }
    }

    pub fn output_channels(&self) -> &[Channel] {
        self.mixer.output_channels()
    }
}
