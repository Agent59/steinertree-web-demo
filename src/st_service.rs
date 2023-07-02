use actix_web::{get, post, web, Responder, Result};
use actix_files::NamedFile;
use serde::Deserialize;

use crate::geosteiner::{
    rs_safe_compute_esmt_vec,
    Point,
};

pub fn st_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/steinertree")
            .service(calc_st)
            .service(st_page)
    );
}

#[derive(Debug, Deserialize)]
pub struct TermsJson {
    terms: Vec<Point>,
}

#[post("/calc_tree")]
async fn calc_st(json: web::Json<TermsJson>) -> Result<impl Responder> {
    let terms = &json.terms;

    let esmt = rs_safe_compute_esmt_vec(terms);
    
    Ok(web::Json(esmt))
}

#[get("")]
async fn st_page() -> Result<NamedFile> {
    let path = "./static/steinertree.html";
    Ok(NamedFile::open(path)?)
}
