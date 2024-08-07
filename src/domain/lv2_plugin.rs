use crate::domain::port::Port;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Lv2Plugin {
    pub index: u16,
    pub name: String,
    pub input_ports: Vec<Port>,
    pub output_ports: Vec<Port>
}
