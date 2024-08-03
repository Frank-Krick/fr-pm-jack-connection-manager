use crate::domain::channel_strip::ChannelStrip;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Mixer {
    pub channels: Vec<ChannelStrip>,
}

impl Mixer {
    pub fn new(channels: &Vec<ChannelStrip>) -> Self {
        Mixer { channels: channels.clone() }
    }
}
