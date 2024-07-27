use crate::domain::channel::Channel;
use crate::domain::lv2_processor::Lv2Processor;

pub struct FrCrossFader {
    pub name: String,
    input_channels: [Channel; 4],
    output_channels: [Channel; 2],
}

impl FrCrossFader {
    pub fn new(name: String) -> FrCrossFader {
        FrCrossFader {
            name: name.clone(),
            input_channels:
            [
                Channel { name: format!("{}:in_1_l", name, ) },
                Channel { name: format!("{}:in_1_r", name, ) },
                Channel { name: format!("{}:in_2_l", name, ) },
                Channel { name: format!("{}:in_2_r", name, ) },
            ],
            output_channels:
            [
                Channel { name: format!("{}:out_l", name) },
                Channel { name: format!("{}:out_r", name) },
            ],
        }
    }
}

impl Lv2Processor for FrCrossFader {
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