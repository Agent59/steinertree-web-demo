use actix_web::{App, HttpServer, get, Result};
use actix_files::NamedFile;

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
            .service(test)
    })
    .bind(("0.0.0.0", 1515))?
    .run()
    .await
}

#[get("/test")]
async fn test() -> Result<NamedFile> {
    let file = NamedFile::open("./src/html_files/steinertree.html")?;
    Ok(file)
}
