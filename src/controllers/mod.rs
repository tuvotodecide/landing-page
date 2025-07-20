use actix_web::{web, guard};
use actix_web::guard::GuardContext;

mod index;
mod about;
mod blog;
mod health;

const LANGS: &[&str] = &["es", "en"];

fn valid_lang(ctx: &GuardContext) -> bool {
    // Construimos la ruta completa como string
    let path = ctx.head().uri.to_string();
    // El split por '/' da ["", "es", "about", ...]; el índice 1 es el lang
    if let Some(code) = path.split('/').nth(1) {
        LANGS.contains(&code)
    } else {
        false
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    // Redirección raíz "/" según Accept‑Language (abajo)
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