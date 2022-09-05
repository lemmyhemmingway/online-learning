use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use serde::Deserialize;

#[derive(Deserialize)]
struct Login {
    email: String,
    password: String
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/login")]
async fn login(user: web::Json<Login>) -> impl Responder {
    if user.email == "admin@admin.com" && user.password == "123" {
        format!("welcome {}", user.email)
    } else {
        format!("go away")
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("running on {}", port);
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new( || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            // hmm prefix
            .service(
                web::scope("/test")
                .route("/hey", web::get().to(manual_hello))
            )            
            .service(hello)
            .service(echo)
            .service(login)
            .service(health_check)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
