use crate::domain::audio_interface::AudioInterface;
use crate::domain::channel::Channel;

pub struct Umc1820 {
    input_channels: [Channel; 4],
    output_channels: [Channel; 16]
}

impl Umc1820 {
    pub fn new() -> Umc1820 {
        Umc1820 {
            input_channels: [
                Channel { name: String::from("system:playback_1") },
                Channel { name: String::from("system:playback_2") },
                Channel { name: String::from("system:playback_3") },
                Channel { name: String::from("system:playback_4") },
            ],
            output_channels: [
                Channel { name: String::from("system:capture_1") },
                Channel { name: String::from("system:capture_2") },
                Channel { name: String::from("system:capture_3") },
                Channel { name: String::from("system:capture_4") },
                Channel { name: String::from("system:capture_5") },
                Channel { name: String::from("system:capture_6") },
                Channel { name: String::from("system:capture_7") },
                Channel { name: String::from("system:capture_8") },
                Channel { name: String::from("system:capture_11") },
                Channel { name: String::from("system:capture_12") },
                Channel { name: String::from("system:capture_13") },
                Channel { name: String::from("system:capture_14") },
                Channel { name: String::from("system:capture_15") },
                Channel { name: String::from("system:capture_16") },
                Channel { name: String::from("system:capture_17") },
                Channel { name: String::from("system:capture_18") },
            ]
        }
    }
}

impl AudioInterface for Umc1820 {
    fn name(&self) -> &'static str {
        "Behringer UMC1820"
    }

    fn input_channels(&self) -> &[Channel] {
        self.input_channels.as_slice()
    }

    fn output_channels(&self) -> &[Channel] {
        self.output_channels.as_slice()
    }
}
