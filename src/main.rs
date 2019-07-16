use std::env;

use actix_web::{App, HttpServer, web};

use bot::api_views::new_message;

pub mod bot;

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/new-message", web::post().to(new_message))
    })
        .bind(("0.0.0.0", get_port()))
        .expect("Can not bind to port")
        .run()
        .unwrap();
}

fn get_port() -> u16 {
    return env::var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse()
        .expect("PORT must be a number");
}



