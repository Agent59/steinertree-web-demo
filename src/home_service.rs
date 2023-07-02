use actix_web::{get, web, Result};
use actix_files::NamedFile;

pub fn home_config(cfg: &mut web::ServiceConfig) {
    cfg.service(home);
}

#[get("/")]
async fn home() -> Result<NamedFile> {
    let path = "./static/home.html";
    Ok(NamedFile::open(path)?)
}
