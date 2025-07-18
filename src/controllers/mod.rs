use actix_web::web;

mod index;
mod about;
mod blog;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/",      web::get().to(index::index))
        .route("/about", web::get().to(about::about))
        .route("/blog",  web::get().to(blog::blog));
}
