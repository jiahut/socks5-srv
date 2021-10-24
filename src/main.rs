use anyhow::Result;
use socks5_proxy::server;

#[tokio::main]
async fn main() -> Result<()> {
    let port = std::env::args().nth(1).unwrap_or("1080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let s = server::new(addr.parse()?, None)?;
    println!("socks5://{}", addr);
    s.run().await?;

    Ok(())
}
