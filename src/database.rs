use mysql::*;
use mysql::prelude::*;
use rand::Rng;

pub fn connec_database(url: String) -> Pool{
    let builder = OptsBuilder::from_opts(Opts::from_url(&url).unwrap());
    let pool = Pool::new(builder.ssl_opts(SslOpts::default())).unwrap();
    println!("Successfully connected to PlanetScale!");
    return pool;
}

pub fn check_user(pool: &Pool, std_id: String) -> bool{
    let mut conn = pool.get_conn().unwrap();
    let result = conn.exec_first::<String, _, _>("SELECT * FROM user WHERE std_id = :std_id", params!{
        "std_id" => std_id
    }).unwrap();
    match result {
        Some(_) => return true,
        None => return false
    }
}

pub fn get_mail_by_std_id(pool: &Pool, std_id: &String) -> String{
    let mut conn = pool.get_conn().unwrap();
    let result = conn.exec_first::<String, _, _>("SELECT email FROM std_data WHERE std_id = :std_id", params!{
        "std_id" => std_id
    }).unwrap();
    match result {
        Some(mail) => return mail,
        None => return "None".to_string()
    }
}

pub fn rand_pin() -> String{
    let mut rng = rand::thread_rng();
    let mut pin: String = rng.gen_range(0..999999).to_string();
    while pin.len() < 6 {
        pin = "0".to_string() + &pin;
    }
    pin
}

pub fn put_pin(pool: &Pool, std_id: &String, pin: &String){
    let mut conn = pool.get_conn().unwrap();
    let result = conn.exec_first::<Row, _, _>("SELECT * FROM std_idTopin WHERE std_id = :std_id", params!{
        "std_id" => std_id
    }).unwrap();
    if result.is_some() {
        conn.exec_drop("UPDATE std_idTopin SET pin = :pin WHERE std_id = :std_id", params!{
            "pin" => pin,
            "std_id" => std_id
        }).unwrap();
    } else {
        conn.exec_drop("INSERT INTO std_idTopin (std_id, pin) VALUES (:std_id, :pin)", params!{
            "std_id" => std_id,
            "pin" => pin
        }).unwrap();
    }
}

pub fn check_pin(pool: &Pool, std_id: &String, pin: &String) -> bool{
    let mut conn = pool.get_conn().unwrap();
    let result = conn.exec_first::<Row, _, _>("SELECT * FROM std_idTopin WHERE std_id = :std_id AND pin = :pin", params!{
        "std_id" => std_id,
        "pin" => pin
    }).unwrap();
    match result {
        Some(_) => return true,
        None => return false
    }
}

pub fn remove_pin(pool: &Pool, std_id: &String){
    let mut conn = pool.get_conn().unwrap();
    conn.exec_drop("DELETE FROM std_idTopin WHERE std_id = :std_id", params!{
        "std_id" => std_id
    }).unwrap();
}

pub fn put_line_id(pool: &Pool, std_id: &String, line_id: &String){
    let mut conn = pool.get_conn().unwrap();
    let result = conn.exec_first::<String, _, _>("SELECT * FROM user WHERE std_id = :std_id", params!{
        "std_id" => std_id
    }).unwrap();
    if result.is_some() {
        conn.exec_drop("UPDATE user SET line_id = :line_id WHERE std_id = :std_id", params!{
            "line_id" => line_id,
            "std_id" => std_id
        }).unwrap();
    } else {
        conn.exec_drop("INSERT INTO user (std_id, line_id) VALUES (:std_id, :line_id)", params!{
            "std_id" => std_id,
            "line_id" => line_id
        }).unwrap();
    }
}