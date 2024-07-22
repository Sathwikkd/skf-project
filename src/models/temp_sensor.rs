
use serde::{Serialize,Deserialize};
use mongodb::bson::{oid::ObjectId};


#[derive(Debug,Serialize,Deserialize)]
pub struct TempSensorData{
    _id:ObjectId,
    temp:i32
}

#[derive(Debug,Serialize,Deserialize)]
pub struct TempSensorDataRequest {
    temp:i32
}

impl TryFrom<TempSensorDataRequest> for TempSensorData {
    type Error=Box<dyn std::error::Error>;

    fn try_from(value: TempSensorDataRequest) -> Result<Self, Self::Error> {
        return Ok(Self {
            _id:ObjectId::new(),
            temp:value.temp
        })
    }
}

