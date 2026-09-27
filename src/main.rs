use std::sync::Mutex;

use actix_web::{App, HttpResponse, HttpServer, Responder, post, web::{self, Json}};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
struct SignupInput {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String
}

#[post("/signup")]
async fn sign_up(body: Json<SignupInput>, app_state: web::Data<AppState>) -> impl Responder {
    let mut users = app_state.users.lock().unwrap();
    let mut user_index = app_state.user_index.lock().unwrap();
    println!("User Index : {}", user_index);

    let user_found = users.iter().find(|user| user.username == body.username);

    if user_found.is_none() {
        
        *user_index = *user_index + 1;

        println!("{}", body.username);
        println!("{}", body.password);

        users.push(User {
            id: *user_index,
            username: body.username.clone(),
            password: body.password.clone()
        });

        println!("{:?}", users);
        drop(users);

        HttpResponse::Ok().json(Response {
            message: String::from("Successfully signed up")
        })
    } else {
        HttpResponse::Unauthorized().json(Response {
            message: String::from("User already exists")
        })
    }

}

#[derive(Debug)]
struct User {
    id: u32,
    username: String,
    password: String
}

struct AppState {
    user_index: Mutex<u32>,
    users: Mutex<Vec<User>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        user_index: Mutex::new(0),
        users: Mutex::new(vec![])
    });
    HttpServer::new(move || {App::new().app_data(app_state.clone()).service(sign_up)}).bind(("127.0.0.1", 3001))?.run().await
}