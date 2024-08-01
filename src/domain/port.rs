#[derive(Debug, Clone)]
pub struct Port {
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
