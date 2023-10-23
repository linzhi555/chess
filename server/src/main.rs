use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpServer, Responder};

use chess_core::{Cmd, Game};

use server::{LoginRequest, LoginResponse};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/game/cmd")]
async fn game_cmd(cmd: web::Json<Cmd>, game: web::Data<Mutex<Game>>) -> impl Responder {
    let mut game = game.lock().unwrap();
    let _ = game.exec_cmd(&cmd);
    println!("{:?}", *game);
    web::Json(game.clone())
}

#[post("/game/state")]
async fn game_state(game: web::Data<Mutex<Game>>) -> impl Responder {
    let game = game.lock().unwrap();
    web::Json(game.clone())
}

#[post("/login")]
async fn login(log_req: web::Json<LoginRequest>) -> impl Responder {
    println!("{:?}", log_req);
    web::Json(LoginResponse {
        ok: true,
        err: "None".to_string(),
        token: "ssss".to_string(),
    })
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let game: web::Data<Mutex<Game>> = web::Data::new(Mutex::new(Game::new()));
    HttpServer::new(move || {
        App::new()
            .service(greet)
            .service(game_cmd)
            .service(game_state)
            .service(login)
            .app_data(game.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
