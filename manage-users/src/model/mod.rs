
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;




#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email : String,
    pub password : String , 
}

