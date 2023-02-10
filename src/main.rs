pub mod mongodb;
pub mod structs;
use crate::structs::{RequestById, UserRequest};
use ::mongodb::{bson::doc, Collection};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "localhost";
    let port = "5510";
    let collection = web::Data::new(mongodb::mongodb_init().await.unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(collection.clone())
            .service(register_handler)
            .service(get_by_id_handler)
    })
    .bind(format!("{}:{}", ip, port))?
    .run()
    .await
}

#[post("/register")]
async fn register_handler(
    json: web::Json<UserRequest>,
    collection: web::Data<Collection<UserRequest>>,
) -> impl Responder {
    println!("{:?}", json);
    // let body=insert_one(&collection, json.into_inner()).await.unwrap();
    let body = match collection.insert_one(json.into_inner(), None).await {
        Ok(body) => body,
        Err(e) => {
            println!("Error al insertar el usuario: {:?}", e.to_string());
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    };

    return HttpResponse::Ok().json(body);
}
#[get("/getById")]
async fn get_by_id_handler(
    json: web::Query<RequestById>,
    collection: web::Data<Collection<UserRequest>>,
) -> impl Responder {
    println!("{:?}", json);
    let id = doc! {"_id": json.id};
    let user = match collection.find_one(id, None).await {
        Ok(user) => user,
        Err(e) => {
            println!("Error al obtener el usuario: {:?}", e.to_string());
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    };
    let user = match user {
        Some(user) => user,
        None => {
            println!("No se encontró el usuario");
            return HttpResponse::NotFound().json("No se encontró el usuario");
        }
    };
    return HttpResponse::Ok().json(user);
}
