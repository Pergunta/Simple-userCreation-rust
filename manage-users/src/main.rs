#[macro_use]
extern crate rocket;

mod crypto_auth;
mod model;
mod routes;
mod database;

use database::*;
use routes::*;

#[launch]
fn rocket() -> _ {
   let db = MongoRepo::init();
   rocket::build()
      .manage(db)
      .mount("/auth", routes![register, login])
      .mount("/user", routes![get_user, update_user])
   
         
}