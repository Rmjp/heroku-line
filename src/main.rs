mod database;
mod mail;
use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use json;
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;
use mysql;
use serde::Deserialize;

static mut pool: Option<mysql::Pool> = None;

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

#[derive(Deserialize)]
struct Info {
    line_id: String,
    std_id: String,
}
#[get("/loginsubmit")]
async fn loginsubmit(info: web::Query<Info>) -> impl Responder {
    println!("{} {}", info.line_id, info.std_id);
    HttpResponse::Ok().body("Ok")
}

async fn index(req: HttpRequest) -> Result<NamedFile, std::io::Error> {
    let path: PathBuf = "./files/login.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
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
            .service(test)
            .service(loginsubmit)
            .route("/login", web::get().to(index))
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}