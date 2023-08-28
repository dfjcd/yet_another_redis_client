use std::str::FromStr;

use resp::ConnectionMultiplexer;

mod resp;
#[tokio::main]
async fn main() {
    let conn = ConnectionMultiplexer {
        user: String::from_str("default").unwrap(),
        url: String::from_str("redis.test.com").unwrap(),
        pass: String::from_str("pass1234").unwrap(),
        tls: false,
        port: 6379,
    };

    println!("Config: {:?}", conn);
}
