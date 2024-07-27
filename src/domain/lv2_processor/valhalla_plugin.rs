use crate::domain::channel::Channel;
use crate::domain::lv2_processor::Lv2Processor;

pub struct ValhallaPlugin {
    name: String,
    input_channels: [Channel; 2],
    output_channels: [Channel; 2]
}

impl ValhallaPlugin {
    pub fn new(name: String) -> ValhallaPlugin {
        ValhallaPlugin {
            name: name.clone(),
            input_channels:
            [
                Channel { name: format!("{}:input_1", name) },
                Channel { name: format!("{}:input_2", name) }
            ],
            output_channels:
            [
                Channel { name: format!("{}:output_1", name) },
                Channel { name: format!("{}:output_2", name) }
            ]
        }
    }
}

impl Lv2Processor for ValhallaPlugin {
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