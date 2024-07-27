use crate::domain::channel::Channel;
use crate::domain::channel_strip::ChannelStrip;
use crate::domain::jack_connection_manager::JackConnectionManager;
use crate::domain::jack_mixer::group_mixer::GroupMixer;
use crate::domain::jack_mixer::JackMixer;

pub struct GroupMixerSection {
    group_mixer: GroupMixer,
    channel_strips: [ChannelStrip; 8]
}

impl GroupMixerSection {
    pub fn new() -> GroupMixerSection {
        GroupMixerSection {
            group_mixer: GroupMixer::new(),
            channel_strips: [
                ChannelStrip::new("_grp_1"),
                ChannelStrip::new("_grp_2"),
                ChannelStrip::new("_grp_3"),
                ChannelStrip::new("_grp_4"),
                ChannelStrip::new("_grp_5"),
                ChannelStrip::new("_grp_6"),
                ChannelStrip::new("_grp_7"),
                ChannelStrip::new("_grp_8"),
            ]
        }
    }

    pub fn connect_internals(&self, jack_connection_manager: &JackConnectionManager) {
        for channel_strip in &self.channel_strips {
            channel_strip.connect_internals(jack_connection_manager);
        }

        for channel_number in 0..7 {
            jack_connection_manager.connect(
                &self.channel_strips[channel_number].output_channels()[0],
                &self.group_mixer.input_channels()[2 * channel_number]
            );
            jack_connection_manager.connect(
                &self.channel_strips[channel_number].output_channels()[1],
                &self.group_mixer.input_channels()[2 * channel_number + 1]
            );
        }
    }

    pub fn channel_inputs(&self, channel_number: usize) -> &[Channel] {
        self.channel_strips[channel_number].input_channels()
    }

    pub fn output_channels(&self) -> &[Channel] {
        self.group_mixer.output_channels()
    }
}