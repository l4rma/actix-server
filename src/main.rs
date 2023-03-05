use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;


#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(hello)
        .service(
            Files::new("/", "./src/static/")
                .show_files_listing()
                .index_file("index.html")
                .use_last_modified(true)))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
