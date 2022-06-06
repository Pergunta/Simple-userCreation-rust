use crate::{model::User, database::MongoRepo};
use mongodb::{bson::oid::ObjectId ,results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};


#[post("/register", data = "<new_user>")]
pub fn register(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };
    let user_detail = db.create_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
    
}

#[get("/user/<path>")]
pub fn get_user(db: &State<MongoRepo>, path: String ) -> Result<Json<User>, Status>{
    let id = path;
    if id.is_empty(){
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user(&id);
    match user_detail{
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }


}

#[put("/user/<path>", data = "<new_user>")]
pub fn update_user(
    db: &State<MongoRepo>,
    path: String,
    new_user: Json<User>,
    )-> Result<Json<User>,Status>{
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };
    let update_result = db.update_user(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id);
                return match updated_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }

}