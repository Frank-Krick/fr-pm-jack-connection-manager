use crate::domain::port::Port;

#[derive(Debug)]
pub struct Lv2Plugin {
    pub index: u16,
    pub(crate) name: String,
    pub(crate) input_ports: Vec<Port>,
    pub(crate) output_ports: Vec<Port>
}
