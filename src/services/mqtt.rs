
use rumqttc::{AsyncClient, ClientError as MQTTError, EventLoop, MqttOptions,QoS,Publish};
use std::time::Duration;
use std::env;

use crate::services::db::DBService;
use crate::models::temp_sensor::{TempSensorData,TempSensorDataRequest};

pub struct MQTTService {
    pub client:AsyncClient,
    pub event_loop:EventLoop
}


impl MQTTService {
    pub async fn init()->Result<MQTTService,MQTTError> {
        let host=env::var("MQTT_HOST").expect("failed to read the MQTT_HOST env variable");
        let port=env::var("MQTT_PORT").expect("failed to read the MQTT_PORT env variable");
        let client_id=env::var("MQTT_CLIENT_ID").expect("failed to read the MQTT_CLIENT_ID env variable");
        let keep_alive=env::var("MQTT_KEEP_ALIVE").expect("failed to read the MQTT_KEEP_ALIVE env variable");
        
        let port:u16=port.trim().parse().expect("port parse error");
        let keep_alive:u64=keep_alive.trim().parse().expect("keep alive parse error");

        let mut mqtt_options=MqttOptions::new(client_id, host, port);
        mqtt_options.set_keep_alive(Duration::from_secs(keep_alive));


        let (client,event_loop)=AsyncClient::new(mqtt_options, 10);

        client.subscribe("sensor/temp", QoS::AtLeastOnce).await?;

        return Ok(Self {
            client:client,
            event_loop:event_loop
        });
    }

    pub async fn handler(&self,db:&DBService,packete:Publish)->Result<(),Box<dyn std::error::Error>> {
        // let topic=packete.topic.as_str();
        let message=String::from_utf8(packete.payload.to_vec())?;

        let temp_sensor_request:TempSensorDataRequest=serde_json::from_str(&message)?;

        let temp_sensor_data=TempSensorData::try_from(temp_sensor_request)?;

        db.insert_temp_data(temp_sensor_data).await?;
        

        return Ok(());
    }
 }