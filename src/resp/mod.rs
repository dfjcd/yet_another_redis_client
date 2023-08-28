use thiserror::Error;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct ConnectionParams {
    pub url: String,
    pub user: String,
    pub pass: String,
    pub port: i32,
    pub tls: bool,
}

struct ConnectionMultiplexer {
    stream: TcpStream,
}

impl ConnectionMultiplexer {
    pub async fn connect_async(conn: &ConnectionParams) -> Result<(), ConnectionError> {
        let _conn = TcpStream::connect(&conn.url).await?;
        return Ok(());
    }
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Unable to connect to redis instance")]
    UnableToConnect(String),
}

impl From<tokio::io::Error> for ConnectionError {
    fn from(value: tokio::io::Error) -> Self {
        ConnectionError::UnableToConnect(value.to_string())
    }
}
