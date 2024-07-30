use pipewire::channel::Sender;
use crate::channel_messages::PipewireCommand;

#[derive(Clone)]
pub struct PipewireService {
    channel_sender: Sender<PipewireCommand>
}

impl PipewireService {
    pub fn new(channel_sender: Sender<PipewireCommand>) -> Self {
        PipewireService {
            channel_sender
        }
    }

    pub fn connect(&self) {
        match self.channel_sender.send(PipewireCommand::Connect {}) {
            Ok(_) => {
                println!("added")
            }
            Err(error) => {
                println!("failed {:?}", error)
            }
        }

    }
}