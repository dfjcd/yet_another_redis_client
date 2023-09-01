use config::{Config, File};
use resp::{ConnectionMultiplexer, ConnectionParams};

mod resp;
mod commands;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let settings = Config::builder()
        .add_source(File::with_name("app\\config.toml"))
        .build()?;

    let param = settings.try_deserialize::<ConnectionParams>()?;

    println!("Config: {:?}", param);

    let mut conn = ConnectionMultiplexer::connect_async(param).await?;
    conn.say_hello().await?;
    Ok(())
}
