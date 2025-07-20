// src/controllers/about.rs
use actix_web::{web, HttpResponse, Result, HttpRequest};
use tera::{Tera, Context};
use crate::i18n::Translations;

pub async fn about(
    tmpl: web::Data<Tera>,
    tr:   web::Data<Translations>,
    req:  HttpRequest,
) -> Result<HttpResponse> {
    let lang = req.match_info().get("lang").unwrap_or("es");
    let t = tr.get(lang).unwrap_or_else(|| tr.get("es").unwrap());

    let mut ctx = Context::new();
    ctx.insert("t", &t);
    ctx.insert("lang", &lang);
    ctx.insert("page_title", t["about"]["page_title"].as_str().unwrap());
    ctx.insert("show_header", &true);

    // enlaces hreflang idénticos al ejemplo de index
    let current_url = req.uri().path();
    let mut alt = Vec::new();
    for code in tr.codes() {
        let url = current_url.replacen(&format!("/{lang}"), &format!("/{code}"), 1);
        alt.push((code.to_string(), url));
    }
    ctx.insert("alt_links", &alt);

    let body = tmpl
        .render("pages/about.html", &ctx)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
