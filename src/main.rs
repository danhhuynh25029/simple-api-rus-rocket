mod model;
mod repository;

use std::vec;
use model::base::Task;
use mongodb::results::InsertOneResult;
use repository::mgo::MongoRepo;
use rocket::{get, http::Status, serde::json::{Json}, State};
use serde::Serialize;
#[macro_use]
extern crate rocket;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}


#[derive(Serialize)]
pub struct ListResponse{
    pub status: String,
    pub result: usize,
    pub data: Vec<Task>
}

#[get("/healthchecker")]
pub async fn health_checker_handler() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Rocket";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

#[get("/task")]
pub async fn task() -> Result<Json<ListResponse>,Status>{
    let mut vec = Vec::new();
    vec.push(Task{
        id : 10,
        title : "Title 1".to_string(),
        content: "Hello mot ngay that tuyet nhat".to_string(),
    });
    let response  = ListResponse{
        status : "sucess".to_string(),
        result : vec.len(),
        data: vec,
    } ;
    Ok(Json(response))
}

#[post("/",data = "<new_task>")]
pub fn create_task(db : &State<MongoRepo>, new_task : Json<Task>) ->Result<Json<InsertOneResult>, Status> {
    let data = Task{
        id : new_task.id,
        title : new_task.title.to_owned(),
        content : new_task.content.to_owned(),
    };

    let task_detail = db.crate_task(data);
    
    match task_detail {
        Ok(task) => Ok(Json(task)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<path>")]
pub fn get_task(db : &State<MongoRepo>,path : String) -> Result<Json<Task>,Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let task_resp = db.get_task(&id);

    match task_resp {
        Ok(task) => Ok(Json(task)),
        Err(_) => Err(Status::BadRequest)
    }
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build().manage(db).mount("/api", routes![health_checker_handler,task,create_task,get_task])
}