use crate::domain::channel::Channel;
use crate::domain::lv2_processor::Lv2Processor;

pub struct CalvAudioPlugin {
    pub name: String,
    input_channels: [Channel; 2],
    output_channels: [Channel; 2]
}

impl CalvAudioPlugin {
    pub fn new(name: String) -> CalvAudioPlugin {
        CalvAudioPlugin {
            name: name.clone(),
            input_channels:
            [
                Channel { name: format!("{}:In L", name) },
                Channel { name: format!("{}:In R", name) },
            ],
            output_channels:
            [
                Channel { name: format!("{}:Out L", name) },
                Channel { name: format!("{}:Out R", name) },
            ]
        }
    }
}

impl Lv2Processor for CalvAudioPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn input_channels(&self) -> &[Channel] {
        self.input_channels.as_slice()
    }

    fn output_channels(&self) -> &[Channel] {
        self.output_channels.as_slice()
    }
}