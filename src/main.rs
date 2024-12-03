use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from responder")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("this is manual")
}

async fn example1() -> impl Responder {
    HttpResponse::Ok().body("example1")
}

async fn example2() -> impl Responder {
    HttpResponse::Ok().body("example2")
}

#[derive(Deserialize)]
struct Register {
    username: String,
    country: String,
}

async fn json_register(form: web::Json<Register>) -> impl Responder {
    format!("Hello {} from {}!", form.username, form.country)
}

#[derive(Deserialize)]
struct Info {
    name: String,
    user_id: u32,
}

#[get("/user/{user_id}/{name}")]
async fn userhome(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
            "Welcome: {}, name: {}",
            info.name, info.user_id
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(userhome)
            .service(
                web::scope("/api")
                .route("/example1", web::get().to(example1))
                .route("/example2", web::get().to(example2))
                .route("/register", web::post().to(json_register)),
            )
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
