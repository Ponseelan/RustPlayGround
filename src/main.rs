
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{time::Instant, thread::{ThreadId, Thread}};
mod Module;
mod ModuleTest;

// install Instant
//https://docs.rs/chrono/0.4.19/chrono/struct.Instant.html



#[derive(Serialize, Deserialize, Debug)]
struct Person  {
    name: String,
    age: String
}

//Create a REST API
#[get("/")]
pub async fn hello() -> impl Responder {
    
    HttpResponse::Ok().body("Hello world!")
}

//API to read data from client
#[get("/person/{name}/{age}")]
pub async fn person(info: actix_web::web::Path<(String,String)>) -> impl Responder {
    let start = Instant::now();
    let user: Person =Person{name:info.0.to_string(),age:info.1.to_string()};
    let duration: std::time::Duration = start.elapsed();
    let response=serde_json::to_string(&user).unwrap();
    println!("Time elapsed in this function is: {:?}", duration);
    if(user.age=="10"){
        return HttpResponse::Ok().body(response)
    }
  HttpResponse::InternalServerError().body(response)
}



//get API for Person in json
#[get("/person_json")]
pub async fn person_json() -> impl Responder {
    const ter:i32=1;
    let user: Person =Person{name:"Rust".to_string(),age:"10".to_string()};
    let json = serde_json::to_string(&user).unwrap();
    HttpResponse::Ok().body(json)
}


//serde_json.to_string usage
//https://docs.serde.rs/serde_json/fn.to_string.html

//Run this API on 8085 port
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        //Register the API
            .service(person)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Rust Fundemanetals
//https://www.youtube.com/watch?v=zF34dRivLOw&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=1

//command to run this API
//cargo run --bin hello_world