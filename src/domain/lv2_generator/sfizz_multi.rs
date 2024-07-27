use crate::domain::channel::Channel;
use crate::domain::lv2_generator::Lv2Generator;

pub struct SFizzMulti {
    output_channels: [Channel; 16],
}

impl SFizzMulti {
    pub(crate) fn new() -> SFizzMulti {
        SFizzMulti {
            output_channels:
            [
                Channel { name: String::from("sfizz-multi:Output 1 Left") },
                Channel { name: String::from("sfizz-multi:Output 1 Right") },
                Channel { name: String::from("sfizz-multi:Output 2 Left") },
                Channel { name: String::from("sfizz-multi:Output 2 Right") },
                Channel { name: String::from("sfizz-multi:Output 3 Left") },
                Channel { name: String::from("sfizz-multi:Output 3 Right") },
                Channel { name: String::from("sfizz-multi:Output 4 Left") },
                Channel { name: String::from("sfizz-multi:Output 4 Right") },
                Channel { name: String::from("sfizz-multi:Output 5 Left") },
                Channel { name: String::from("sfizz-multi:Output 5 Right") },
                Channel { name: String::from("sfizz-multi:Output 6 Left") },
                Channel { name: String::from("sfizz-multi:Output 6 Right") },
                Channel { name: String::from("sfizz-multi:Output 7 Left") },
                Channel { name: String::from("sfizz-multi:Output 7 Right") },
                Channel { name: String::from("sfizz-multi:Output 8 Left") },
                Channel { name: String::from("sfizz-multi:Output 8 Right") },
            ]
        }
    }
}

impl Lv2Generator for SFizzMulti {
    fn name(&self) -> &'static str {
        "SFizz"
    }

    fn output_channels(&self) -> &[Channel] {
        self.output_channels.as_slice()
    }
}
