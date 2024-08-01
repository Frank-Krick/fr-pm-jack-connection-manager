use crate::channel_messages::PipewireCommand;
use pipewire::channel::Sender;

#[derive(Clone)]
pub struct PipewireService {
    channel_sender: Sender<PipewireCommand>,
}

impl PipewireService {
    pub fn new(channel_sender: Sender<PipewireCommand>) -> Self {
        PipewireService { channel_sender }
    }

    pub fn connect(
        &self,
        output_node_id: String,
        output_port_id: String,
        input_node_id: String,
        input_port_id: String,
    ) {
        match self.channel_sender.send(PipewireCommand::Connect(
            output_node_id,
            output_port_id,
            input_node_id,
            input_port_id,
        )) {
            Ok(_) => {
                println!("added")
            }
            Err(error) => {
                println!("failed {:?}", error)
            }
        }
    }
}
