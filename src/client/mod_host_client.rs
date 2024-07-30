use std::io::{Read, Write};
use std::net::TcpStream;

pub struct ModHostClient {
    stream: TcpStream
}

impl ModHostClient {
    pub fn new(stream: TcpStream) -> ModHostClient {
        ModHostClient { stream }
    }

    pub fn add_plugin(&mut self, lv2_plugin_uri: &str, mod_plugin_index: u16) -> Result<u16, String> {
        let message = format!("add {} {}\0", lv2_plugin_uri, mod_plugin_index);
        self.stream.write_all(message.as_bytes()).unwrap();
        let mut buffer = [0; 120];
        match self.stream.read(&mut buffer) {
            Ok(_response) => {
                let message = String::from_utf8(
                    buffer.into_iter().filter(|c| *c != b'\0').collect::<Vec<u8>>()).unwrap();

                if message[0..4].to_string() != "resp" {
                    return Err(format!("received incorrectly formatted response {}", message[0..4].to_string()));
                }

                return match message[5..].parse::<i32>() {
                    Ok(code) => {
                        if code < 0 {
                            println!("received error code {}", code);
                            Err(code.to_string())
                        } else {
                            Ok(code as u16)
                        }
                    }
                    Err(error) => {
                        Err(format!("received incorrectly formatted argument {}", error))
                    }
                }
            },
            Err(error) => Err(format!("Couldn't read from stream {}", error))
        }
    }
}
