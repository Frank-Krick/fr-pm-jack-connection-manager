use crate::domain::channel::Channel;
use crate::domain::channel_strip::ChannelStrip;
use crate::domain::jack_connection_manager::JackConnectionManager;
use crate::domain::lv2_processor::fr_cross_fader::FrCrossFader;
use crate::domain::lv2_processor::Lv2Processor;
use crate::domain::sooper_looper::SooperLooper;

pub struct LooperFaderStrips {
    looper: SooperLooper,
    fader: FrCrossFader,
    channel_strip: ChannelStrip
}

impl LooperFaderStrips {
    pub fn new(channel_number: u8) -> LooperFaderStrips {
        LooperFaderStrips {
            looper: SooperLooper::new(channel_number - 1),
            fader: FrCrossFader::new(format!("st_fr_cf_{}", channel_number)),
            channel_strip: ChannelStrip::new(format!("{}", channel_number).as_str())
        }
    }

    pub fn connect_internals(&self, jack_connection_manager: &JackConnectionManager) {
        jack_connection_manager.connect(
            &self.looper.output_channels()[0],
            &self.fader.input_channels()[2]
        );
        jack_connection_manager.connect(
            &self.looper.output_channels()[1],
            &self.fader.input_channels()[3]
        );
        jack_connection_manager.connect(
            &self.fader.output_channels()[0],
            &self.channel_strip.input_channels()[0]
        );
        jack_connection_manager.connect(
            &self.fader.output_channels()[1],
            &self.channel_strip.input_channels()[1]
        );
    }

    pub fn input_channels(&self) -> [&Channel; 4] {
        [
            &self.fader.input_channels()[0],
            &self.fader.input_channels()[1],
            &self.looper.input_channels()[0],
            &self.looper.input_channels()[1]
        ]
    }

    pub fn output_channels(&self) -> &[Channel] {
        self.channel_strip.output_channels()
    }
}