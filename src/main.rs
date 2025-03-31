use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use std::io;

#[derive(Serialize, Deserialize)]
struct Matrix {
    rows: usize,
    columns: usize,
    data: Vec<Vec<i32>>,
}

impl Matrix {
    fn new(rows: usize, columns: usize, data: Vec<Vec<i32>>) -> Self {
        Self {
            rows,
            columns,
            data,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MatrixInput {
    mat1: Matrix,
    mat2: Matrix,
}

#[get("/")]
async fn root(message: web::Data<String>) -> impl Responder {
    message.get_ref().clone()
}

#[get("/matrix/{row}/{column}")]
async fn matrix(params: web::Path<(String, String)>) -> Result<impl Responder, actix_web::Error> {
    let row: usize = params.0.parse().map_err(|_| {
        actix_web::error::ErrorBadRequest("Invalid row parameter. Must be a positive integer.")
    })?;
    let col: usize = params.1.parse().map_err(|_| {
        actix_web::error::ErrorBadRequest("Invalid column parameter. Must be a positive integer.")
    })?;

    println!("Requesting row: {}, col: {}", row, col);

    let data: Vec<Vec<i32>> = vec![vec![0; col]; row];
    let response: Matrix = Matrix::new(row, col, data);
    Ok((Json(response), StatusCode::OK))
}

#[post("/matrix/add")]
async fn add_matrices(input: web::Json<MatrixInput>) -> Result<impl Responder, actix_web::Error> {
    let mat1: &Matrix = &input.mat1;
    let mat2: &Matrix = &input.mat2;

    if mat1.rows != mat2.rows || mat1.columns != mat2.columns {
        return Err(actix_web::error::ErrorBadRequest("Incompatible matrices."));
    }

    let mut result: Vec<Vec<i32>> = vec![vec![0; mat1.columns]; mat1.rows];
    for i in 0..mat1.rows {
        for j in 0..mat1.columns {
            result[i][j] = mat1.data[i][j] + mat2.data[i][j];
        }
    }

    let response: Matrix = Matrix::new(mat1.rows, mat1.columns, result);
    Ok(Json(response))
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
            .service(add_matrices)
    })
    .bind((host, port))?
    .run();

    println!("Server is running on {host}:{port}");
    server.await
}
