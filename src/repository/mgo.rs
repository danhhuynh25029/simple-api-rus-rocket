use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult,
    sync::{Client, Collection},
};

use crate::model::base::Task;
pub struct MongoRepo{
    col: Collection<Task>,
}

impl MongoRepo {
    pub fn init() -> Self {
        let uri:&str = "mongodb://localhost:27017";
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("local");
        let col: Collection<Task> = db.collection("task");
        MongoRepo { col }
    }

    pub fn crate_task(&self,new_task:Task) -> Result<InsertOneResult,Error>{
        let task = self.col.insert_one(new_task, None).ok().expect("Error creating user");
        Ok(task)
    }
}