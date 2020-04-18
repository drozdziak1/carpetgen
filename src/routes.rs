use actix_files::NamedFile;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::path::PathBuf;

#[derive(Clone, Default, Deserialize, Serialize)]
struct Carpet {
    pub id: i32,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub w: f64, // longitude
    pub h: f64, // latitude
}

#[get("/carpets")]
async fn carpets() -> impl Responder {
    let carpets: Vec<Carpet> = vec![
        Carpet {
            id: 1,
            name: "My carpet".to_owned(),
            x: 5.0,
            y: 5.0,
            w: 5.01,
            h: 5.01,
        },
        Default::default(),
    ];
    HttpResponse::Ok().json(carpets)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    //cfg.service(index);
    cfg.service(carpets);
}
