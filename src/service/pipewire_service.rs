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
        let message = PipewireCommand::Connect(
            output_node_id,
            output_port_id,
            input_node_id,
            input_port_id,
        );
        println!("Sending message:{message:#?}");
        self.channel_sender.send(message).expect("Couldn't send message to channel")
    }
}
