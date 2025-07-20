// src/controllers/about.rs
use actix_web::{web, HttpResponse, Result, HttpRequest};
use tera::{Tera, Context};
use crate::i18n::Translations;
use actix_web::dev::ConnectionInfo;

const PAGE: &str = "about";      //name of the language directory   

pub async fn about(
    tmpl: web::Data<Tera>,
    tr:   web::Data<Translations>,
    req:  HttpRequest,
        conn: ConnectionInfo,
) -> Result<HttpResponse> {
    let lang = req.match_info().get("lang").unwrap_or("es");
    let t = tr.get(PAGE, lang).unwrap_or_else(|| tr.get(PAGE, "es").unwrap());

    let scheme_host = format!("{}://{}", conn.scheme(), conn.host());  // usa ConnectionInfo :contentReference[oaicite:4]{index=4}
    let canonical_url = format!("{scheme_host}{}", req.uri().path());
    let default_url   = format!("{scheme_host}/es/");

    let mut ctx = Context::new();
    ctx.insert("t", &t);
    ctx.insert("lang", &lang);
    ctx.insert("canonical_url", &canonical_url);
    ctx.insert("default_url",   &default_url);
    ctx.insert("show_header", &true);

    let mut alt = Vec::new();
    for code in tr.langs_for(PAGE) {
        let alt_url = canonical_url.replacen(&format!("/{lang}"), &format!("/{code}"), 1);
        alt.push((code.to_string(), alt_url));
    }
    ctx.insert("alt_links", &alt);

    let body = tmpl
        .render("pages/about.html", &ctx)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
