mod models;
mod services;

use crate::services::db::DBService;
use crate::services::mqtt::MQTTService;

use dotenv;


#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    //loading the env file to the environment
    dotenv::dotenv().ok();

    let db=DBService::init().await?;
    let mut mqtt=MQTTService::init().await?;

    loop {
        if let Ok(message)=mqtt.event_loop.poll().await {
            if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(packete))=message {
                mqtt.handler(&db, packete).await?;
            }
        }
    }
}
