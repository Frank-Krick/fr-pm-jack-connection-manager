use crate::domain::port::Port;

#[derive(Debug)]
#[allow(dead_code)]
pub struct PipewireDevice {
    pub name: String,
    pub factory_id: String,
    pub client_id: String,
    pub description: String,
    pub nick: String,
    pub media_class: String,
    pub object_serial: String,
}

#[derive(Debug, Clone)]
pub struct PipewirePort {
    pub id: String,
    pub name: String,
    pub direction: String,
    pub physical: String,
    pub alias: String,
    pub group: String,
    pub path: String,
    pub dsp_format: String,
    pub node_id: String,
    pub audio_channel: String
}

impl Port {
    pub fn from(pipewire_port: &PipewirePort) -> Port {
        Port {
            id: pipewire_port.id.clone(),
            name: pipewire_port.name.clone(),
            direction: pipewire_port.direction.clone(),
            physical: pipewire_port.physical.clone(),
            alias: pipewire_port.alias.clone(),
            group: pipewire_port.group.clone(),
            path: pipewire_port.path.clone(),
            dsp_format: pipewire_port.dsp_format.clone(),
            node_id: pipewire_port.node_id.clone(),
            audio_channel: pipewire_port.audio_channel.clone()
        }
    }
}
