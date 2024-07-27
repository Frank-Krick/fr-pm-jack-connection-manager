use crate::domain::channel::Channel;
use crate::domain::jack_mixer::JackMixer;

pub struct GroupMixer {
    name: String,
    input_channels: Vec<Channel>,
    output_channels: Vec<Channel>,
}

impl GroupMixer {
    pub fn new() -> GroupMixer {
        GroupMixer {
            name: String::from("group_channels"),
            input_channels:
            ["Drums", "Bass", "Melody", "Atmos", "Return 1", "Return 2", "Return 3", "Return 4"].iter().map(|channel_name| {
                [
                    Channel { name: format!("group_channels:{} L", channel_name) },
                    Channel { name: format!("group_channels:{} R", channel_name) },
                ]
            }).flatten().collect(),
            output_channels:
            ["Main"].iter().map(|channel_name| {
                [
                    Channel { name: format!("group_channels:{} L", channel_name) },
                    Channel { name: format!("group_channels:{} R", channel_name) },
                ]
            }).flatten().collect(),
        }
    }
}

impl JackMixer for GroupMixer {
    fn name(&self) -> &str {
        &self.name
    }

    fn input_channels(&self) -> &[Channel] {
        self.input_channels.as_slice()
    }

    fn output_channels(&self) -> &[Channel] {
        self.output_channels.as_slice()
    }

    fn input_channels_by_name(&self, name: &str) -> Vec<&Channel> {
        self.input_channels.iter().filter(|c| {
            c.name.contains(name)
        }).collect()
    }

    fn output_channels_by_name(&self, name: &str) -> Vec<&Channel> {
        self.output_channels.iter().filter(|c| {
            c.name.contains(name)
        }).collect()
    }
}
