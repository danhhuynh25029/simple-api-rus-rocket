mod model;
mod repository;
mod handler;

use std::sync::atomic::{AtomicUsize};
use std::vec;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::response::status;
use rocket::{Request, Data};

use crate::repository::mgo::MongoRepo;
use crate::handler::task;
#[macro_use]
extern crate rocket;
struct AuthenMiddleware;
#[rocket::async_trait]
impl  Fairing for AuthenMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "Authentication Middleware",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let auth_token = request.headers().get_one("Authorization");
        match auth_token{
            Some(token) => println!("{}",token),
            _ => print!("")
        }
    }
}

#[launch]
fn rocket() -> _ {
    let db: MongoRepo = MongoRepo::init();
    rocket::build().attach(AuthenMiddleware).manage(db).mount("/api", routes![task::create_task,task::get_task])
}