#[macro_use]
extern crate rocket;

 
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
      .mount("/", routes![register])
      .mount("/", routes![get_user])
      .mount("/",routes![update_user])
   
         
}