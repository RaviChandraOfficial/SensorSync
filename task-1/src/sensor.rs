use serde::{Deserialize, Serialize};

// define a struct for the request body
#[derive(Deserialize)]
pub struct Request {
    pub id: i32,
    pub sensor_name: String,
    pub data: String,
    pub location: String,
}



// define a struct for the query parameters
#[derive(Deserialize)]
pub struct Query {
    pub id: i32,
}


#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct NoteModel {
    pub id: i32,
    pub sensor_name: String,
    pub location :String,
    pub data :String,
}


#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NoteModelResponse {
    pub id: i32,
    pub sensor_name: String,
    pub location: String,
    pub data : String
}


#[derive(Deserialize, Serialize)]
pub struct Getid {
    pub id:i32
}