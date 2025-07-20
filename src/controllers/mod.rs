use actix_web::web;

mod index;
mod about;
mod blog;
mod health;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/",      web::get().to(index::index))
        .route("/health",      web::get().to(health::health))
        .route("/about", web::get().to(about::about))
        .route("/blog",  web::get().to(blog::blog));
}
