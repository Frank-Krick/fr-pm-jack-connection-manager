use std::sync::mpsc::Sender;
use crate::channel_messages::ModHostCommand;

pub struct ModHostService {
    mod_host_channel_sender: Sender<ModHostCommand>
}

impl ModHostService {
    pub fn new(mod_host_channel_sender: Sender<ModHostCommand>) -> ModHostService {
        ModHostService { mod_host_channel_sender }
    }

    pub fn add_lv2_plugin(&mut self, lv2_plugin_uri: String) {
        match self.mod_host_channel_sender.send(ModHostCommand::Add(lv2_plugin_uri)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}
