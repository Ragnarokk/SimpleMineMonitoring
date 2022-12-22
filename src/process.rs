use std::io::Write;
use std::io::Read;
use std::process::{Command, Stdio};
use std::{thread, time};

fn main() {
    let mut child = Command::new("python3")
        .arg("pipe.py")
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");
    
    let mut stdout = child.stdout.take().expect("Failed to open stdout");
    let mut out = std::io::stdout();
    std::thread::spawn(move || loop {
        let mut buf = [100];
        match stdout.read(&mut buf) {
            Err(err) => {
                println!("Error reading from stdout : {}", err);
                break;
            }
            Ok(count) => {
                if count >= 1 {
                    out.write_all(&buf).expect("Can't write to stdout");
                    out.flush().expect("Can't flush stdout");
                }
            }
        }
    });
    
    match child.wait(){
        Ok(_) => {
            println!("Server exited succesfully")
        }
        Err(_) => {
            println!("Server crashed");
        }
    };
}
