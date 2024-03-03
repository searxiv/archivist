use actix_web::Responder;

pub async fn ping() -> impl Responder {
    "Pong!"
}
