use std::vec;
use rocket::{get, http::Status, serde::json::Json};
use serde::Serialize;
use my_crate::Task;
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![health_checker_handler,task])
}