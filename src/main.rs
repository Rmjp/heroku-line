use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use json;

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
use std::env;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
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