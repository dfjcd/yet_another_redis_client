use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

use thiserror::Error;

#[derive(Debug, serde::Deserialize)]
pub struct ConnectionParams {
    pub url: String,
    pub user: String,
    pub pass: String,
    pub port: i32,
    pub tls: bool,
}

impl ConnectionParams {
    fn url(&self) -> String {
        format!("{}:{}", &self.url, &self.port)
    }
}

pub struct ConnectionMultiplexer {
    stream: TcpStream,
    param: ConnectionParams,
}

impl ConnectionMultiplexer {
    pub async fn connect_async(param: ConnectionParams) -> Result<Self, anyhow::Error> {
        let conn = TcpStream::connect(param.url())?;
        let _ = conn.set_ttl(60);
        return Ok(ConnectionMultiplexer {
            stream: conn,
            param: param,
        });
    }

    pub async fn execute(command: String) {

    }

    pub async fn say_hello(&mut self) -> Result<(), anyhow::Error> {
        println!("{:?}", self.stream);
        let _ = &self.stream.write_all(
            format!("HELLO 2 AUTH {} {}\r\n", &self.param.user, &self.param.pass).as_bytes(),
        )?;
        let mut reader: BufReader<&TcpStream> = BufReader::new(&self.stream);
        //let mut line = String::new();
        // let mut buf: Vec<u8> = Vec::new();
        // let lines = reader.lines();

        // for line in lines {
        //     println!("Line: {:?}", line.unwrap());
        // }
        loop {
            let mut buf = String::new();
            let result = reader.read_line(&mut buf)?;
            if result == 0 {
                break;
            }
            println!("Received {} bytes", result);
            println!("Line: {:?}", buf);
        }
        let _ = self.stream.shutdown(std::net::Shutdown::Both);
        Ok(())
    }
}
