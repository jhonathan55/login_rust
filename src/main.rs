pub mod structs;
use actix_web::{web, App, HttpServer, post, HttpResponse, Responder};
use crate::structs::UserRequest;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "localhost";
    let port = "5510";
    HttpServer::new(|| App::new().service(register_handler))
        .bind(format!("{}:{}", ip, port))?
        .run()
        .await
}

#[post("/register")]
async fn register_handler(json: web::Json<UserRequest>) -> impl Responder {
    println!("{:?}", json);

    return HttpResponse::Ok().json(json.into_inner());
}
