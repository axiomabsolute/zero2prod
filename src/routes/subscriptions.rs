use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{types::uuid, PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscriptionsInputForm {
    email: String,
    name: String,
}

pub async fn subscriptions(
    form: web::Form<SubscriptionsInputForm>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    if form.email.is_empty() {
        return HttpResponse::BadRequest()
            .reason("Email field is required")
            .finish();
    }
    if form.name.is_empty() {
        return HttpResponse::BadRequest()
            .reason("Name field is required")
            .finish();
    }

    let mutation_result = sqlx::query!(
        r#"
            insert into subscriptions (id, email, name, subscribed_at)
            values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await;

    match mutation_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
