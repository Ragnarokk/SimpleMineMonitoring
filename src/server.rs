pub mod monitoring {
    tonic::include_proto!("monitoring");
}

mod process;
mod log_handler;

use futures::Stream;
use std::{pin::Pin, time::Duration, net::ToSocketAddrs};
use std::sync::{Arc, Mutex};
use prost_types::Timestamp;
use tonic::{transport::Server, Request, Response, Status};
use monitoring::{ConsoleLogRequest, ConsoleLogResponse, 
                 StartMinecraftRequest, StartMinecraftResponse};
use process::{ServerStatus, Process};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use log_handler::LogHandler;

#[derive(Default)]
pub struct MonitoringServer {
    process: Arc<Mutex<Process>>,
    log_handler: Arc<Mutex<LogHandler>>,
}

#[tonic::async_trait]
impl monitoring::monitoring_server::Monitoring for MonitoringServer {

    type ConsoleLogStream = Pin<Box<dyn Stream<Item = Result<ConsoleLogResponse, Status>> + Send>>;

    async fn console_log(
        &self, 
        request: Request<ConsoleLogRequest>
    ) -> Result<Response<Self::ConsoleLogStream>, Status> {
        println!("Communication etablie !");
        println!("Client connected from {:?}", request.remote_addr());

        let handler = Arc::clone(&self.log_handler);
        let mut stream = Box::pin(tokio_stream::iter(std::iter::from_fn(move || {
            match String::from_utf8(handler.lock().unwrap().take_logs()) {
                Ok(logs) => {
                    Some(ConsoleLogResponse {
                        log: logs
                    })
                },
                Err(_) => None
            }
        })).throttle(Duration::from_millis(200)));

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {},
                    Err(_item) => { break; }
                }
            }
            println!("Client disconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::ConsoleLogStream
        ))
    }

    async fn start_minecraft(
        &self, 
        _request: Request<StartMinecraftRequest>
    ) -> Result<Response<StartMinecraftResponse>, Status> {

        let timestamp = Timestamp {
            seconds: chrono::offset::Local::now().timestamp(),
            nanos: 0,
        };

        match self.process.lock().unwrap().status {
            ServerStatus::STOPPED => {
                let handler = Arc::clone(&self.log_handler);
                let process = Arc::clone(&self.process);
                let x = match process.lock().unwrap().start(handler) {
                    true => {
                        Ok(Response::new(StartMinecraftResponse{timestamp: Some(timestamp)}))
                    },
                    false => Err(Status::aborted("Process failed to start"))
                }; x
            },
            ServerStatus::RUNNING => Err(Status::already_exists("The process is already running"))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = MonitoringServer::default();
    Server::builder()
        .add_service(monitoring::monitoring_server::MonitoringServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    Ok(())
}