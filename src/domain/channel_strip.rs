use std::fmt::Debug;

use crate::domain::lv2_plugin::Lv2Plugin;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ChannelStrip {
    pub saturator: Lv2Plugin,
    compressor: Lv2Plugin,
    equalizer: Lv2Plugin,
    pub gain: Lv2Plugin
}

impl ChannelStrip {
    pub fn new(
        saturator: Lv2Plugin,
        compressor: Lv2Plugin,
        equalizer: Lv2Plugin,
        gain: Lv2Plugin
    ) -> Self {
        ChannelStrip { saturator, compressor, equalizer, gain }
    }
}
