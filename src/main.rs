mod controllers;
mod i18n;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Templates
    let tera = Tera::new("templates/**/*").expect("Templates not found");
    let tr = i18n::load("i18n")?;

    println!("Servidor en http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(tera.clone()))
        .app_data(web::Data::new(tr.clone()))
            .configure(controllers::init)
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
