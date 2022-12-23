use std::io::{Read};
use std::process::{Command, Stdio, Child};
use std::sync::{Arc, Mutex};

use crate::log_handler::LogHandler;

const BUFFER_SIZE: usize = 100;

pub enum ServerStatus {
    RUNNING,
    STOPPED
}

impl Default for ServerStatus {
    fn default() -> Self { ServerStatus::STOPPED }
}

#[derive(Default)]
pub struct Process {
    pub status: ServerStatus,
    pub process: Option<Child>,
}

impl Process {
    pub fn start(&mut self, handler: Arc<Mutex<LogHandler>>) -> bool {
        match Command::new("python3")
            .arg("pipe.py")
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() {
            
            Ok(mut child) => {
                let mut stdout = child.stdout.take().expect("Failed to open stdout");
                self.process = Some(child);
                self.status = ServerStatus::RUNNING;

                // let mut out = std::io::stdout();
                let handler = Arc::clone(&handler);
                std::thread::spawn(move || {
                    let mut great_buffer: Vec<u8> = Vec::new();
                    loop {
                        let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
                        match stdout.read(&mut buf) {
                            Err(err) => {
                                println!("Error reading from stdout : {}", err);
                                break;
                            }
                            Ok(count) => {
                                match count {
                                    0 => {},
                                    BUFFER_SIZE => {
                                        great_buffer.extend_from_slice(&buf);
                                    },
                                    _ => {
                                        let mut curr_buffer = Vec::from(buf);
                                        if !great_buffer.is_empty() {
                                            curr_buffer.append(&mut great_buffer);
                                            great_buffer = Vec::new();
                                        }

                                        handler.lock().unwrap().add_logs(curr_buffer);
                                    }
                                }
                            }
                        }
                    }
                });

                true
            },
            Err(_) => false
        }
    }

    // fn read(&self) -> Result<&u8> {
    //     match self.process.stdout.take() {
    //         Ok(stdout) => {
    //             stdout.read()
    //         }
    //         Err(_) => {
    //             println!("Failed to open stdout");

    //             Error
    //         }
    //     }
    // }
}
