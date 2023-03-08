use crate::app::state::*;

use actix_web::{
    get,
    web::Data,
    Responder,
    HttpResponse
};

#[get("/")]
pub async fn get_api(data: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("Counter is {}", data.counter.lock().unwrap()))
}

#[get("/add")]
pub async fn get_api_add(data: Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body("Added to counter")
}

#[get("/sub")]
pub async fn get_api_sub(data: Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter -= 1;
    HttpResponse::Ok().body("Subtracted from counter")
}
