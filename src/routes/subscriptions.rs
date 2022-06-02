use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct SubscriptionsInputForm {
    email: String,
    name: String
}

pub async fn subscriptions(_form: web::Form<SubscriptionsInputForm>) -> HttpResponse {
    if _form.email.is_empty() {
        return HttpResponse::BadRequest().reason("Email field is required").finish()
    }
    if _form.name.is_empty() {
        return HttpResponse::BadRequest().reason("Name field is required").finish()
    }
    HttpResponse::Ok().finish()
}
