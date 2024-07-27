use crate::domain::channel::Channel;

pub struct SooperLooper {
    name: String,
    input_channels: [Channel; 2],
    output_channels: [Channel; 2],
}

impl SooperLooper {
    pub fn new(loop_number: u8) -> SooperLooper {
        SooperLooper {
            name: String::from("sooperlooper"),
            input_channels:
            [
                Channel { name: format!("sooperlooper:loop{}_in_1", loop_number) },
                Channel { name: format!("sooperlooper:loop{}_in_1", loop_number) },
            ],
            output_channels:
            [
                Channel { name: format!("sooperlooper:loop{}_out_1", loop_number) },
                Channel { name: format!("sooperlooper:loop{}_out_1", loop_number) },
            ],
        }
    }

    pub fn input_channels(&self) -> &[Channel] {
        self.input_channels.as_slice()
    }

    pub fn output_channels(&self) -> &[Channel] {
        self.output_channels.as_slice()
    }
}
