use crate::domain::channel::Channel;
use crate::domain::jack_mixer::JackMixer;

pub struct SendMixer {
    name: String,
    input_channels: Vec<Channel>,
    output_channels: Vec<Channel>
}

impl SendMixer {
    pub fn new(send_name: &str) -> SendMixer {
        let mixer_name = format!("send_{}", send_name);
        SendMixer {
            name: mixer_name.clone(),
            input_channels:
            ["DSMPL", "DFire", "DEuro", "Prophet", "System 1", "SE02", "S4", "ZYN", "09", "10"].iter().map(|channel_name| {
                [
                    Channel { name: format!("{}:{} L", mixer_name, channel_name) },
                    Channel { name: format!("{}:{} R", mixer_name, channel_name) },
                ]
            }).flatten().collect(),
            output_channels:
            ["Main L", "Main R"].iter().map(|channel_name| {
                [
                    Channel { name: format!("{}:{} L", mixer_name, channel_name) },
                    Channel { name: format!("{}:{} R", mixer_name, channel_name) },
                ]
            }).flatten().collect(),
        }
    }
}

impl JackMixer for SendMixer {
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
