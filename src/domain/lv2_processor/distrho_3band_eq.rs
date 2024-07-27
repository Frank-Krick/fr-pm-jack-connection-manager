use crate::domain::channel::Channel;
use crate::domain::lv2_processor::Lv2Processor;

pub struct Distrho3BandEq {
    pub name: String,
    input_channels: [Channel; 2],
    output_channels: [Channel; 2]
}

impl Distrho3BandEq {
    pub fn new(name: String) -> Distrho3BandEq {
        Distrho3BandEq {
            name: name.clone(),
            input_channels:
            [
                Channel { name: format!("{}:Audio Input 1", name) },
                Channel { name: format!("{}:Audio Input 2", name) }
            ],
            output_channels:
            [
                Channel { name: format!("{}:Audio Output 1", name) },
                Channel { name: format!("{}:Audio Output 2", name) }
            ]
        }
    }
}
impl Lv2Processor for Distrho3BandEq {
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