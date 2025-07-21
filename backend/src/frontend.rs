use actix_web::{HttpResponse, Responder, web};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../frontend/build"]
pub struct Frontend;

pub async fn get_frontend(path: web::Path<String>) -> impl Responder {
    if let Some(content) = Frontend::get(path.as_str()) {
        return HttpResponse::Ok()
            .content_type(
                mime_guess::from_path(path.as_str())
                    .first_or_octet_stream()
                    .as_ref(),
            )
            .body(content.data.into_owned());
    }

    if let Some(content) = Frontend::get(&format!("{path}/index.html")) {
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(content.data.into_owned());
    }

    if let Some(content) = Frontend::get("index.html") {
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(content.data.into_owned());
    }

    HttpResponse::NotFound().into()
}
