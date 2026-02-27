#![warn(clippy::all, clippy::pedantic)]

use actix_web::{App, HttpServer, web};
use env_logger::Env;
use log::info;

use crate::kv_store::KeyValueStore;

mod assets;
mod config;
mod file_store;
mod health;
mod kv_store;
mod proxy;
mod runtime;
mod script;
mod test_endpoint;
mod view;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::from_env();

    env_logger::Builder::from_env(Env::default().default_filter_or(config.log_level())).init();

    info!("{config}");

    let bind_address = config.adress();
    let data = web::Data::new(KeyValueStore::new());
    let config_data = web::Data::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .app_data(config_data.clone())
            .service(assets::assets)
            .service(health::health)
            .service(test_endpoint::test_target)
            .service(kv_store::get_kv)
            .service(kv_store::set_kv)
            .service(kv_store::delete_kv)
            .service(file_store::upload_file)
            .service(file_store::download_file)
            .service(file_store::delete_file)
            .service(script::run)
            .service(view::index)
            .service(view::key_value::kv_page)
            .service(view::file_store::files_page)
            .service(view::file_store::upload_file_ui)
            .service(view::file_store::delete_file_ui)
            .service(view::proxy::proxy_page)
            .service(view::script::script_page)
            .default_service(web::route().to(proxy::default_proxy))
    })
    .bind(bind_address)?
    .run()
    .await
}
