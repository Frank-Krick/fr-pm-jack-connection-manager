use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::mpsc::Sender;

use anyhow::Result;

use crate::channel_messages::ModHostCommand;

#[derive(Clone)]
pub struct ModHostService {
    channel_sender: Sender<ModHostCommand>,
    next_plugin_index: Arc<AtomicU16>
}

impl ModHostService {
    pub fn new(channel_sender: Sender<ModHostCommand>) -> ModHostService {
        ModHostService { channel_sender, next_plugin_index: Arc::new(AtomicU16::new(0)) }
    }

    pub fn add_lv2_plugin(&self, lv2_plugin_uri: String) -> Result<u16> {
        let index = self.next_plugin_index.fetch_add(1, Ordering::Relaxed);
        match self.channel_sender.send(
            ModHostCommand::Add(lv2_plugin_uri, index)) {
            Ok(_) => { Ok(index) }
            Err(error) => { Err(anyhow::Error::new(error)) }
        }
    }
}
