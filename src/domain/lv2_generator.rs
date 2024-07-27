pub mod sfizz_multi;

use crate::domain::channel::Channel;

pub trait Lv2Generator {
    fn name(&self) -> &'static str;
    fn output_channels(&self) -> &[Channel];
}