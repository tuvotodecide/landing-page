use actix_web::{web, HttpRequest, HttpResponse, Result};
use accept_language::{intersection};
use tera::{Tera, Context};
use crate::i18n::Translations;
use actix_web::dev::ConnectionInfo;

const SUPPORTED: &[&str] = &["es", "en"];
const PAGE: &str = "index";  //name of the language directory

/* ---------- / (raíz) REDIRECT ---------- */
pub async fn root_redirect(req: HttpRequest) -> HttpResponse {
    let header_val = req
        .headers()
        .get(actix_web::http::header::ACCEPT_LANGUAGE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let common = intersection(header_val, SUPPORTED);
    let best = common.get(0).map(|s| s.as_str()).unwrap_or("es");                             // default
    HttpResponse::Found()
        .append_header((actix_web::http::header::LOCATION, format!("/{}/", best)))
        .finish()
}

pub async fn index(
    tmpl: web::Data<Tera>,
    tr:   web::Data<Translations>,
    req:  HttpRequest,
    conn: ConnectionInfo,          
) -> Result<HttpResponse> {
    let lang = req.match_info().get("lang").unwrap_or("es");
    let t = tr.get(PAGE, lang).unwrap_or_else(|| tr.get(PAGE, "es").unwrap());

    /* Canonical/Alternate */
    let scheme_host = format!("{}://{}", conn.scheme(), conn.host());  // usa ConnectionInfo :contentReference[oaicite:4]{index=4}
    let canonical_url = format!("{scheme_host}{}", req.uri().path());
    let default_url   = format!("{scheme_host}/es/");

    /* Contexto Tera */
    let mut ctx = Context::new();
    ctx.insert("t", &t);
    ctx.insert("lang", &lang);
    ctx.insert("canonical_url", &canonical_url);
    ctx.insert("default_url",   &default_url);
    ctx.insert("show_header", &true);

    /* hreflang alternos */
    let mut alt = Vec::new();
    for code in tr.langs_for(PAGE) {
        let alt_url = canonical_url.replacen(&format!("/{lang}"), &format!("/{code}"), 1);
        alt.push((code.to_string(), alt_url));
    }
    ctx.insert("alt_links", &alt);

    let body = tmpl.render("pages/index.html", &ctx)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
