#![warn(clippy::all, clippy::pedantic)]

use actix_web::{App, HttpServer, web};
use env_logger::Env;
use log::info;

mod assets;
mod config;
mod health;
mod kv_store;
mod view;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::from_env();

    env_logger::Builder::from_env(Env::default().default_filter_or(config.log_level())).init();

    info!("{config}");

    let data = web::Data::new(kv_store::new());

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(assets::assets)
            .service(health::health)
            .service(kv_store::get_kv)
            .service(kv_store::set_kv)
            .service(kv_store::delete_kv)
            .service(view::index)
            .service(view::key_value::kv_page)
            .service(view::proxy::proxy_page)
            .service(view::script::script_page)
    })
    .bind(config.adress())?
    .run()
    .await
}
