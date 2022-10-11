mod database;
mod mail;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use json;
use dotenv::dotenv;
use std::env;
use mysql::*;

static mut pool: Option<Pool> = None;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("{}",req_body);
    HttpResponse::Ok().body(req_body)
}

#[post("/test")]
async fn test(req_body: String) -> impl Responder {
    let req_json: json::JsonValue = json::parse(&req_body).unwrap();
    match database::check_user(& unsafe{pool.clone()}.unwrap(), req_json["originalDetectIntentRequest"]["payload"]["data"]["source"]["userId"].to_string()) {
        true => {},
        false => return HttpResponse::Ok().body("false")
    }

    let result = json::object!{
    text: {
        text: [
            "test test hello"
                ]
        },
        platform: "LINE"
    };
    HttpResponse::Ok().body(json::stringify(json::object!{ fulfillmentMessages: [result] }))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    unsafe {pool = Some(
        database::connec_database(env::var("DATABASE_URL").expect("DATABASE_URL. not found"))
    );}
    let PORT:u16 = env::var("PORT").unwrap_or("8080".to_string()).parse().unwrap();
    println!("I am ready!");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(test)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}