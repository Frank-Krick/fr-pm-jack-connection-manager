pub mod umc_1820;

use crate::domain::channel::Channel;

pub trait AudioInterface {
    fn name(&self) -> &'static str;
    fn input_channels(&self) -> &[Channel];
    fn output_channels(&self) -> &[Channel];
}
