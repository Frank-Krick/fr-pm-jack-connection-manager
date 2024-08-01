pub enum ModHostCommand {
    Add(String, u16)
}

#[derive(Debug)]
pub enum PipewireCommand {
    Connect(String, String, String, String)
}
