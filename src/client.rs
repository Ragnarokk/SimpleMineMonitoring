pub mod monitoring {
    tonic::include_proto!("monitoring");
}

use monitoring::monitoring_client::MonitoringClient;
use monitoring::{StartMinecraftRequest, ConsoleLogRequest};
use prost_types::Timestamp;
use std::sync::{Arc, Mutex};
use tonic::transport::Channel;
use tokio_stream::StreamExt;

fn get_time() -> Timestamp {
    Timestamp {
        seconds: chrono::offset::Local::now().timestamp(),
        nanos: 0,
    }
}

async fn read(recv: Arc<Mutex<MonitoringClient<Channel>>>) {
    let stream = recv.lock().unwrap()
        .console_log(ConsoleLogRequest {
            timestamp: Some(get_time())
        })
        .await
        .unwrap()
        .into_inner();
    
    loop {
        let mut other_stream = stream.take(5);
        while let Some(chars) = stream.next().await {
            print!("{}", chars.unwrap().log)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(Mutex::new(MonitoringClient::connect("http://[::1]:50051").await.unwrap()));

    let request = tonic::Request::new(StartMinecraftRequest {
        timestamp: Some(get_time())
    });

    let response = client.lock().unwrap().start_minecraft(request).await?;
    println!("RESPONSE: {:?}", response);

    let recv = Arc::clone(&client);
    read(recv);

    Ok(())
}