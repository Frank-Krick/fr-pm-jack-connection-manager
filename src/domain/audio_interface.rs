use crate::domain::port::Port;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AudioInterface {
    pub name: String,
    pub input_ports: Vec<Port>,
    pub output_ports: Vec<Port>
}
