use actix_web::{get, Responder};

#[utoipa::path(
    responses(
        (status = 200, description = "Something like healthcheck")
    )
)]
#[get("/ping")]
pub async fn ping() -> impl Responder {
    "Pong!"
}
