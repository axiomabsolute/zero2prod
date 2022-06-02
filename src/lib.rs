use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct SubscriptionsInputForm {
    email: String,
    name: String
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<SubscriptionsInputForm>) -> HttpResponse {
    if _form.email.is_empty() {
        return HttpResponse::BadRequest().reason("Email field is required").finish()
    }
    if _form.name.is_empty() {
        return HttpResponse::BadRequest().reason("Name field is required").finish()
    }
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
        )
        .listen(listener)?
        .run();

    Ok(server)
}
