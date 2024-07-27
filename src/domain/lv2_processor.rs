pub mod calv_audio_plugin;
pub mod distrho_3band_eq;
pub mod fr_cross_fader;
pub mod valhalla_plugin;

use crate::domain::channel::Channel;

pub trait Lv2Processor {
    fn name(&self) -> &str;
    fn input_channels(&self) -> &[Channel];
    fn output_channels(&self) -> &[Channel];
}
