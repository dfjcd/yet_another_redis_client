use async_trait::async_trait;
use crate::resp::ConnectionMultiplexer;

#[async_trait]
pub trait Command {
    async fn execute(&mut self, connection: ConnectionMultiplexer);
}

pub struct HelloCommand;

#[async_trait]
impl Command for HelloCommand {
    async fn execute(&mut self, connection: ConnectionMultiplexer) {
        connection.execute(String::from("HELLO 3")).await;
    }
}
