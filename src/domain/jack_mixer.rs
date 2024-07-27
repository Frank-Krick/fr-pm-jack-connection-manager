pub mod input_mixer;
pub mod group_mixer;
pub mod drum_mixer;
pub mod send_mixer;

use crate::domain::channel::Channel;

pub trait JackMixer {
    fn name(&self) -> &str;
    fn input_channels(&self) -> &[Channel];
    fn output_channels(&self) -> &[Channel];
    fn input_channels_by_name(&self, name: &str) -> Vec<&Channel>;
    fn output_channels_by_name(&self, name: &str) -> Vec<&Channel>;
}
