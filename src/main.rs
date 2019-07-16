extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use futures::Stream;
use telegram_bot::*;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();

    let token = String::from("857986614:AAGxbElOOyvqDhdl9f8Re78PcRPru8JX1HU");
    let api = Api::configure(token).build(core.handle()).unwrap();

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {
        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                // Answer message with "Hi".
                api.spawn(message.text_reply(format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                )));
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}

//use std::env;
//
//use actix_web::{App, HttpServer, web};
//
//use bot::api_views::new_message;
//
//pub mod bot;
//
//fn main() {
//    HttpServer::new(|| {
//        App::new()
//            .route("/new-message", web::post().to(new_message))
//    })
//        .bind(("0.0.0.0", get_port()))
//        .expect("Can not bind to port")
//        .run()
//        .unwrap();
//}
//
//fn get_port() -> u16 {
//    return env::var("PORT")
//        .unwrap_or_else(|_| String::from("3000"))
//        .parse()
//        .expect("PORT must be a number");
//}



