use std::result::Result;
use std::error::Error;
use config::{Config, File, FileFormat};
use mysql::*;
use mysql::prelude::*;


fn init_db() -> Result<PooledConn, Box<dyn Error>> {
    let settings = Config::builder()
        .add_source(File::new("config/MySQL.toml", FileFormat::Toml))
        .build()?;

    let mysql_url = settings.get_string("mysql_url")?;
    let pool = Pool::new(mysql_url.as_str())?;
    let mut conn = pool.get_conn()?;

    Ok(conn)
}

#[test]
fn test_init_db() -> Result<(), Box<dyn Error>> {
    let conn = init_db()?;
    assert!(true);
    Ok(())
}
