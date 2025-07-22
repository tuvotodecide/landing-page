use actix_web::{web, guard};
use actix_web::guard::GuardContext;

mod index;
mod about;
mod blog;
mod health;

const LANGS: &[&str] = &["es", "en"];

fn valid_lang(ctx: &GuardContext) -> bool {
    // build a complete uri as string
    let path = ctx.head().uri.to_string();
    // Split by '/' da ["", "es", "about", ...]; index 1 is lang
    if let Some(code) = path.split('/').nth(1) {
        LANGS.contains(&code)
    } else {
        false
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    // Root redirection "/" according Accept‑Language 
    cfg.route("/", web::get().to(index::root_redirect));

    // scope /{lang}/...
    cfg.service(
        web::scope("/{lang}")
            .guard(guard::fn_guard(valid_lang))
            .route("/",      web::get().to(index::index))
            .route("/about", web::get().to(about::about))
            .route("/blog",  web::get().to(blog::blog))
            .route("/health", web::get().to(health::health))
    );
}