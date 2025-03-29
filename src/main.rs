use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn root() -> impl Responder {
    "欢迎来到 rust web 服务器!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host: &str = "127.0.0.1";
    let port: u16 = 8080;


    let server = HttpServer::new(|| App::new()
        .service(root)
    )
        .bind((host, port))?
        .run();

    println!("Server is running on {host}:{port}");
    server.await
}
