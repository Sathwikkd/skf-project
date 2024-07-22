use mongodb::results::InsertOneResult;
use mongodb::{Client,Collection};
use mongodb::bson::doc;
use std::env;

use crate::models::temp_sensor::TempSensorData;

pub struct DBService {
    pub temp_collection:Collection<TempSensorData>
}

impl DBService {
    pub async fn init()->mongodb::error::Result<Self> {
        //loading the env variable 
        let uri=env::var("DB_URI").expect("failed to read the DB_URI env variable");
        let db_name=env::var("DB_NAME").expect("failed to read the DB_NAME env variable");
        
        //create the client and connect to the server
        let client= Client::with_uri_str(uri).await?;

        let database=client.database(&db_name);

        let sensor_collection:Collection<TempSensorData>=database.collection("sensors");

        client.database(&db_name).run_command(doc! {
            "ping":1
        }).await?;

        println!("connected to mongodb");

        return  Ok(Self {
            temp_collection:sensor_collection
        });
    }

    pub async fn insert_temp_data(&self,temp_data:TempSensorData)->mongodb::error::Result<InsertOneResult>{
        let res=self.temp_collection.insert_one(temp_data).await?;
        println!("temp sensor data inserted successfully");
        return Ok(res);
    }


}