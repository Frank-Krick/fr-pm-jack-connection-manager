use itertools::PadUsing;

#[derive(Debug)]
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
