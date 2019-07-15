use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::env;

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn new_message(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn main() {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/new-message", web::get().to(new_message))
    })
        .bind(("0.0.0.0", port))
        .expect("Can not bind")
        .run()
        .unwrap();
}

//curl -F "url=https://shrouded-hollows-67609.herokuapp.com/new-message"  https://api.telegram.org/bot857986614:AAGxbElOOyvqDhdl9f8Re78PcRPru8JX1HU/setWebhook

