use actix_web::{Result, HttpResponse, web};
use tera::{Tera, Context};

pub async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut ctx = Context::new();
    ctx.insert("page_title", "Inicio");
    // ctx.insert("extra_css", &vec!["/static/css/home.css"]);
    // ctx.insert("extra_js",  &vec!["/static/js/home.js"]);
    ctx.insert("show_header", &true); 

    // aquí el body específico de index
    ctx.insert("body_content", "¡Bienvenido a la página principal!");

    let rendered = tmpl.render("pages/index.html", &ctx)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}
