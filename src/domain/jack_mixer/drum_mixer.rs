use crate::domain::channel::Channel;
use crate::domain::jack_mixer::JackMixer;

pub struct DrumMixer {
    name: String,
    input_channels: Vec<Channel>,
    output_channels: Vec<Channel>,
}

impl DrumMixer {
    pub fn new() -> DrumMixer {
        DrumMixer {
            name: String::from("drum_channels"),
            input_channels:
            ["Kick", "Snare", "Hat", "Open Hat", "Tom", "Cowbell", "Clap", "Cymbal"].iter().map(|channel_name| {
                [
                    Channel { name: format!("drum_channels:{} L", channel_name) },
                    Channel { name: format!("drum_channels:{} R", channel_name) },
                ]
            }).flatten().collect(),
            output_channels:
            ["Kick", "Snare", "Hat", "Open Hat", "Tom", "Cowbell", "Clap", "cymbal"].iter().map(|channel_name| {
                [
                    Channel { name: format!("drum_channels:{} Out L", channel_name) },
                    Channel { name: format!("drum_channels:{} Out R", channel_name) },
                ]
            }).flatten().collect(),
        }
    }
}

impl JackMixer for DrumMixer {
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