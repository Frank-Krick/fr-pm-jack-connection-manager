use crate::domain::channel::Channel;
use crate::domain::jack_mixer::JackMixer;

pub struct InputMixer {
    name: String,
    input_channels: Vec<Channel>,
    output_channels: Vec<Channel>,
}

impl InputMixer {
    pub fn new() -> InputMixer {
        InputMixer {
            name: String::from("input_channels"),
            input_channels:
            ["DSMPL", "DFire", "DEuro", "Prophet", "System 1", "SE02", "S4", "ZYN", "09", "10"].iter().map(|channel_name| {
                [
                    Channel { name: format!("input_channels:{} L", channel_name) },
                    Channel { name: format!("input_channels:{} R", channel_name) },
                ]
            }).flatten().collect(),
            output_channels:
            ["DSMPL", "DFire", "DEuro", "Prophet", "System 1", "SE02", "S4", "ZYN", "09", "10"].iter().map(|channel_name| {
                [
                    Channel { name: format!("input_channels:{} Out L", channel_name) },
                    Channel { name: format!("input_channels:{} Out R", channel_name) },
                ]
            }).flatten().collect(),
        }
    }
}

impl JackMixer for InputMixer {
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