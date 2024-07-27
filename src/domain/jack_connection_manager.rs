use crate::domain::channel::Channel;

pub struct JackConnectionManager {}

impl JackConnectionManager {
    pub fn new() -> JackConnectionManager {
        JackConnectionManager {}
    }

    pub fn connect(&self, source: &Channel, dest: &Channel) {
        println!("connecting {} to {}", source.name, dest.name)
    }
}
