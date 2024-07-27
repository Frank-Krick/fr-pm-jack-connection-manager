use crate::domain::channel::Channel;
use crate::domain::jack_connection_manager::JackConnectionManager;
use crate::domain::jack_mixer::input_mixer::InputMixer;
use crate::domain::jack_mixer::JackMixer;
use crate::domain::jack_mixer::send_mixer::SendMixer;
use crate::domain::looper_fader_strip::LooperFaderStrips;

pub struct MainInputMixerChannel {
    channels: [LooperFaderStrips; 10],
    input_mixer: InputMixer,
    send_mixer: [SendMixer; 5]
}

impl MainInputMixerChannel {
    pub fn new() -> MainInputMixerChannel {
        MainInputMixerChannel {
            channels: [
                LooperFaderStrips::new(1),
                LooperFaderStrips::new(2),
                LooperFaderStrips::new(3),
                LooperFaderStrips::new(4),
                LooperFaderStrips::new(5),
                LooperFaderStrips::new(6),
                LooperFaderStrips::new(7),
                LooperFaderStrips::new(8),
                LooperFaderStrips::new(9),
                LooperFaderStrips::new(10),
            ],
            input_mixer: InputMixer::new(),
            send_mixer: [
                SendMixer::new("1"),
                SendMixer::new("2"),
                SendMixer::new("3"),
                SendMixer::new("4"),
                SendMixer::new("T"),
            ]
        }
    }

    pub fn connect_internals(&self, jack_connection_manager: &JackConnectionManager) {
        for channel in &self.channels {
            channel.connect_internals(jack_connection_manager);
        }

        for channel_number in 0..10 {
            jack_connection_manager.connect(
                &self.channels[channel_number].output_channels()[0],
                &self.input_mixer.input_channels()[2 * channel_number]
            );
            jack_connection_manager.connect(
                &self.channels[channel_number].output_channels()[1],
                &self.input_mixer.input_channels()[2 * channel_number + 1]
            );

            for send_mixer in &self.send_mixer {
                jack_connection_manager.connect(
                    &self.channels[channel_number].output_channels()[0],
                    &send_mixer.input_channels()[2 * channel_number]
                );
                jack_connection_manager.connect(
                    &self.channels[channel_number].output_channels()[1],
                    &send_mixer.input_channels()[2 * channel_number + 1]
                );
            }
        }
    }

    pub fn channel_inputs(&self, channel_number: usize) -> [&Channel; 4] {
        self.channels[channel_number].input_channels()
    }

    pub fn send_mixer(&self, channel_index: u8) -> &SendMixer {
        &self.send_mixer[channel_index as usize]
    }

    pub fn mixer(&self) -> &InputMixer {
        &self.input_mixer
    }
}
