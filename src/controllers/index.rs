use actix_web::{http::header, web, HttpRequest, HttpResponse, Result};
use accept_language::parse;
use tera::{Tera, Context};
use crate::i18n::Translations;

const SUPPORTED: &[&str] = &["es", "en"];

pub async fn root_redirect(req: HttpRequest) -> HttpResponse {
    let header_val = req
        .headers()
        .get(header::ACCEPT_LANGUAGE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let lang = parse(header_val)
        .iter()
.find_map(|l| {
    SUPPORTED
      .iter()
      .find(|&&s| l.matches(s).next().is_some())  
      .copied()
})        .unwrap_or("es");
    HttpResponse::Found()
        .append_header((header::LOCATION, format!("/{}/", lang)))
        .finish()
}


pub async fn index(
    tmpl: web::Data<Tera>,
    tr:   web::Data<Translations>,
    req:  HttpRequest,
) -> Result<HttpResponse> {
    let lang = req.match_info().get("lang").unwrap_or("es");
    let t = tr.get(lang).unwrap_or_else(|| tr.get("es").unwrap());

    let mut ctx = Context::new();
    ctx.insert("t", &t);
    ctx.insert("lang", &lang);
    ctx.insert("page_title", t["page_title"].as_str().unwrap());
    ctx.insert("show_header", &true);

    // hreflang links...
    let current_url = req.uri().path();
    let mut alt = Vec::new();
    for code in tr.codes() {
        let url = current_url.replacen(&format!("/{lang}"), &format!("/{code}"), 1);
        alt.push((code.to_string(), url));
    }
    ctx.insert("alt_links", &alt);

    let body = tmpl
        .render("pages/index.html", &ctx)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
