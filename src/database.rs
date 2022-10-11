use mysql::*;
use mysql::prelude::*;

pub fn connec_database(url: String) -> PooledConn{
    let builder = OptsBuilder::from_opts(Opts::from_url(&url).unwrap());
    let pool = Pool::new(builder.ssl_opts(SslOpts::default())).unwrap();
    let _conn = pool.get_conn().unwrap();
    println!("Successfully connected to PlanetScale!");
    return _conn;
}
