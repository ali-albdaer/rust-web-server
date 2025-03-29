use actix_web::{get, web, App, HttpServer, Responder};
use std::env;
use std::io;

#[get("/")]
async fn root(message: web::Data<String>) -> impl Responder {
    message.get_ref().clone()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host: &str = "127.0.0.1";
    let port: u16 = 8080;
    let args: Vec<String> = env::args().collect();

    let message: String = if args.len() > 1 {
        args[1].clone()
    } else {
        let mut input: String = String::new();
        println!("Enter a message to display on the server: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input.trim().to_string()
    };

    let server: actix_web::dev::Server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(message.clone()))
            .service(root)
    })
    .bind((host, port))?
    .run();

    println!("Server is running on {host}:{port}");
    server.await
}
