use crate::model::base::Task;
use mongodb::results::InsertOneResult;
use crate::repository::mgo::MongoRepo;
use rocket::{get, http::Status, serde::json::{Json}, State};

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