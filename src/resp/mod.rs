use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

use thiserror::Error;

#[derive(Debug)]
pub struct ConnectionParams {
    pub url: String,
    pub user: String,
    pub pass: String,
    pub port: i32,
    pub tls: bool,
}

pub struct ConnectionMultiplexer {
    stream: TcpStream,
    param: ConnectionParams,
}

impl ConnectionMultiplexer {
    pub async fn connect_async(param: ConnectionParams) -> Result<Self, ConnectionError> {
        let addr = format!("{}:{}", &param.url, &param.port);
        let conn = TcpStream::connect(addr)?;
        let _ = conn.set_ttl(60);
        return Ok(ConnectionMultiplexer {
            stream: conn,
            param: param,
        });
    }

    pub async fn say_hello(mut self) -> Result<(), ConnectionError> {
        /*let i = self.stream.write_all(b"HELLO 3");
        println!("write_all: {:?}", i);
        let _ = self.stream.flush();*/
        let addr = format!("{}:{}", &self.param.url, &self.param.port);
        let mut conn = TcpStream::connect(addr)?;
        let _ = conn.write_all(b"HELLO 3\r\n");
        println!("{:?}", conn);
        //let mut buff = Vec::<u8>::new();
        //let mut buf = String::new();
        let mut reader = BufReader::new(&conn);
        let mut line = String::new();

        let result = reader.read_line(&mut line)?;

        println!("Received {} bytes", result);
        println!("Line: {:?}", line);
        let _ = conn.shutdown(std::net::Shutdown::Both);
        Ok(())
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
