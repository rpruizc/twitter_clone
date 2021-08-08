#[macro_use]
extern crate actix_web;

use actix_web::{App, HttpServer, middleware};
use std::{env, io};

mod constants;
mod like;
mod response;
mod tweet;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug, actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(tweet::list)
            .service(tweet::get)
            .service(tweet::create)
            .service(tweet::delete)
            .service(like::list)
            .service(like::plus_one)
            .service(like::minus_one)

    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
