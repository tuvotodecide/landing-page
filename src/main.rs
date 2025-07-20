mod controllers;

use actix_files::Files;
use actix_web::{App, HttpServer, web};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Templates
    let tera = Tera::new("templates/**/*").expect("Templates not found");

    println!("Servidor en http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .configure(controllers::init)
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
