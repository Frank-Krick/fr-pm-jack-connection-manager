use crate::domain::port::Port;

pub trait Lv2Plugin {
    fn index() -> u16;
    fn name() -> String;
    fn input_ports() -> Vec<Port>;
    fn output_ports() -> Vec<Port>;
}
