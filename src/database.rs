use mysql::*;
use mysql::prelude::*;

pub fn connec_database(url: String) -> Pool{
    let builder = OptsBuilder::from_opts(Opts::from_url(&url).unwrap());
    let pool = Pool::new(builder.ssl_opts(SslOpts::default())).unwrap();
    println!("Successfully connected to PlanetScale!");
    return pool;
}

pub fn check_user(pool: &Pool, std_id: String) -> bool{
    let mut conn = pool.get_conn().unwrap();
    let result = conn.exec_first::<String, _, _>("SELECT * FROM users WHERE user_id = :user_id", params!{
        "user_id" => std_id
    }).unwrap();
    match result {
        Some(_) => return true,
        None => return false
    }
}