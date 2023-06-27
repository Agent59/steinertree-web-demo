use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub fn home_config(cfg: &mut web::ServiceConfig) {
    cfg.service(home);
}

#[get("/")]
async fn home() -> impl Responder {     
    "HOME"
}
