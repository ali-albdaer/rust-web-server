use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{get, web, App, HttpServer, Responder};
use std::env;
use std::io;
use serde::Serialize;


#[derive(Serialize)]
struct Matrix {
    row: u32,
    column: u32
}

impl Matrix {
    fn new(row: u32, col: u32) -> Self {
        Self { row: row, column: col }
    }
}

#[get("/")]
async fn root(message: web::Data<String>) -> impl Responder {
    message.get_ref().clone()
}

#[get("/matrix/{row}/{column}")]
async fn matrix(params: web::Path<(String, String)>) -> impl Responder {
    let row: u32 = params.0.parse().unwrap();
    let col: u32 = params.1.parse().unwrap();
    // let row = params.0.parse::<u32>().unwrap(); // Turbofish!

    let response: Matrix = Matrix::new(row, col);
    (Json(response), StatusCode::OK)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host: &str = "127.0.0.1";
    let port: u16 = 8080;
    let message: String = env::args().skip(1).collect::<Vec<String>>().join(" ");

    let message: String = if message.is_empty() {
        let mut input: String = String::new();
        println!("Enter a message to display on the server: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input.trim().to_string()
    } else {
        message
    };

    let server: actix_web::dev::Server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(message.clone()))
            .service(root)
            .service(matrix)
    })
    .bind((host, port))?
    .run();

    println!("Server is running on {host}:{port}");
    server.await
}
