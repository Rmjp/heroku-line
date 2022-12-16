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

static mut DOMAIN: String = String::new();
static mut POOL: Option<mysql::Pool> = None;


#[post("/test")]
async fn test(req_body: String) -> impl Responder {
    match json::parse(&req_body){
        Err(e)=>{
            return HttpResponse::BadRequest().body(format!("Error: {}", e));
        },
        Ok(req_json)=>{
            match database::check_user(& unsafe{POOL.clone()}.unwrap(), req_json["originalDetectIntentRequest"]["payload"]["data"]["source"]["userId"].to_string()) {
                true => {
                    let menu = req_json["queryResult"]["intent"]["displayName"].to_string();
                },
                false => {
                    let mut result = json::object!{
                        "payload": {
                            "line": {
                                "contents": {
                                    "contents": [
                                        {
                                            "type": "bubble",
                                            "body": {
                                                "type": "box",
                                                "contents": [
                                                    {
                                                        "wrap": true,
                                                        "text": "กรุณากรอกเลขประจำตัวนักศึกษา เพื่อเชื่อมต่อบัญชี",
                                                        "type": "text"
                                                    }
                                                ],
                                                "layout": "horizontal"
                                            },
                                            "footer": {
                                                "contents": [
                                                    {
                                                        "action": {
                                                            "uri": "",
                                                            "type": "uri",
                                                            "label": "กดเลย"
                                                        },
                                                        "type": "button",
                                                        "style": "primary"
                                                    }
                                                ],
                                                "type": "box",
                                                "layout": "horizontal"
                                            }
                                        }
                                    ],
                                    "type": "carousel"
                                },
                                "altText": "This is a Flex Message",
                                "type": "flex"
                            }
                        },
                        "platform": "LINE"
                    
                    };
                    result["payload"]["line"]["contents"]["contents"][0]["footer"]["contents"][0]["action"]["uri"] = format!("{}/login?line_id={}", unsafe{DOMAIN.clone()}, req_json["originalDetectIntentRequest"]["payload"]["data"]["source"]["userId"]).into();
                    return HttpResponse::Ok().body(json::stringify(json::object!{ fulfillmentMessages: [result] }));
                }
            }
        }
    }
    
    HttpResponse::Ok().body(req_body)
}

#[derive(Deserialize)]
struct InfoLoginsubmit {
    line_id: String,
    std_id: String,
}
#[post("/loginsubmit")]
async fn loginsubmit(info: web::Json<InfoLoginsubmit>) -> impl Responder {
    println!("{} {}", info.std_id, info.line_id);
    let user_mail = database::get_mail_by_std_id(& unsafe{POOL.clone()}.unwrap(), &info.std_id);
    let pin = database::rand_pin();
    println!("{:?}", user_mail);

    if user_mail == "None" {
        return HttpResponse::Ok().body(json::stringify(json::object!{ status: "Mail not found." }))
    }

    match database::put_pin(& unsafe{POOL.clone()}.unwrap(), &info.std_id, &pin){
        Ok(()) => {},
        Err(str) => {return HttpResponse::Ok().body(json::stringify(json::object!{ status: str }))}
    }

    mail::send_mail_verify(&user_mail, &pin).await.unwrap();
    HttpResponse::Ok().body(json::stringify(json::object!{ status: "OK" }))
}

#[derive(Deserialize)]
struct InfoVerify {
    line_id: String,
    std_id: String,
    pin: String,
}
#[post("/verify")]
async fn verify(info: web::Json<InfoVerify>) -> impl Responder {
    match database::check_pin(& unsafe{POOL.clone()}.unwrap(), &info.std_id, &info.pin) {
        true => {
            database::remove_pin(& unsafe{POOL.clone()}.unwrap(), &info.std_id);
            database::put_line_id(& unsafe{POOL.clone()}.unwrap(), &info.std_id, &info.line_id);
            HttpResponse::Ok().body(json::stringify(json::object!{ status: "OK" }))
        },
        false => HttpResponse::Ok().body(json::stringify(json::object!{ status: "Wrong PIN" }))
    }
}

async fn index(_req: HttpRequest) -> Result<NamedFile, std::io::Error> {
    let path: PathBuf = "./files/login.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn css(_req: HttpRequest) -> Result<NamedFile, std::io::Error> {
    let path: PathBuf = "./files/style.css".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn error(_req: HttpRequest) -> Result<NamedFile, std::io::Error> {
    let path: PathBuf = "./files/error.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    unsafe {
        DOMAIN = env::var("DOMAIN").unwrap();
        POOL = Some(
            database::connec_database(env::var("DATABASE_URL").expect("DATABASE_URL. not found"))
        );
    }

    println!("{:?}", json::Null["test"]);

    let PORT:u16 = env::var("PORT").unwrap_or("8080".to_string()).parse().unwrap();
    println!("I am ready! on port : {}", PORT);

    HttpServer::new(|| {
        App::new()
            .service(test)
            .service(loginsubmit)
            .route("/login", web::get().to(index))
            .route("/style.css", web::get().to(css))
            .service(verify)
            .default_service(web::get().to(error))
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}