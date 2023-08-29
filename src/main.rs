use std::str::FromStr;

use resp::{ConnectionParams, ConnectionError, ConnectionMultiplexer};

mod resp;

#[tokio::main]
async fn main() -> Result<(), ConnectionError> {
    let conn = ConnectionParams {
        user: String::from_str("default").unwrap(),
        url: String::from_str("").unwrap(),
        pass: String::from_str("").unwrap(),
        tls: false,
        port: 6379,
    };

    println!("Config: {:?}", conn);

    let conn = ConnectionMultiplexer::connect_async(conn).await?;
    conn.say_hello().await?;
    Ok(())
}
