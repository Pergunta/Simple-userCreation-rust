use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult , UpdateResult},
    sync::{Client, Collection},
};
use crate::model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            email: new_user.email,
            password: new_user.password,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)

    }

    pub fn get_user(&self,id:&String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"id":obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())

    }

    pub fn update_user(&self , id : &String , new_user : User) -> Result<UpdateResult , Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"id":obj_id};
        let new_doc = doc!{
            "$set":
                {
                    "id":new_user.id,
                    "email":new_user.email,
                    "password":new_user.password,
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc , None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }
  
}