use actix_web::{App, HttpServer, Result, get};
use actix_files::{Files, NamedFile};

use steinertree_web_demo::{
    st_service::st_config,
    home_service::home_config,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(st_config)
            .configure(home_config)
            .service(default_style)
            .service(Files::new("/images", "static/images").show_files_listing())
    })
    .bind(("0.0.0.0", 1515))?
    .run()
    .await
}

#[get("/default.css")]
async fn default_style() -> Result<NamedFile> {
    let path = "./static/default.css";
    Ok(NamedFile::open(path)?)
}
