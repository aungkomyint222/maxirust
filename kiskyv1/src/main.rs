use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;

#[derive(Deserialize)]
struct Input {
    message: String,
}

#[derive(Serialize)]
struct Output {
    response: String,
}

#[post("/")]
async fn handle_input(input: web::Json<Input>) -> impl Responder {
    let message = input.message.to_lowercase();
    
    let response = match message.as_str() {
        "hey" | "hello" | "hi" => {
            let greetings = vec![
                "Hey! How are you?",
                "Hello there!",
                "Hi! What's up?",
                "Hey! Nice to see you!",
                "Hello! How's your day going?"
            ];
            greetings.choose(&mut rand::thread_rng())
                    .unwrap_or(&"hello")
                    .to_string()
        },
        "goodbye" | "bye" | "see you" | "byebye" => {
            let farewells = vec![
                "Goodbye! Take care!",
                "See you later!",
                "Bye! Have a great day!",
                "Until next time!",
                "Take care, come back soon!"
            ];
            farewells.choose(&mut rand::thread_rng())
                    .unwrap_or(&"goodbye")
                    .to_string()
        },
        _ => format!(" {}", input.message)
    };
    
    HttpResponse::Ok().json(Output { response })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting kiskyv1 server at http://localhost:8081");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // Allow all origins, methods, and headers
            .service(handle_input)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}