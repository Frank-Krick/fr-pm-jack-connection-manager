use crate::domain::channel::Channel;

pub struct ZynAddSubFx {
    name: String,
    output_channels: [Channel; 2],
}

impl ZynAddSubFx {
    pub fn new() -> ZynAddSubFx {
        ZynAddSubFx {
            name: String::from("zynaddsubfx"),
            output_channels: [
                Channel { name: String::from("zynaddsubfx:out_1") },
                Channel { name: String::from("zynaddsubfx:out_2") }
            ]
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn output_channels(&self) -> &[Channel] {
        self.output_channels.as_slice()
    }
}