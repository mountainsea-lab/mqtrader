use anyhow::Result;
use mqtrader::nautilus::bot::mqtrader::MqTrader;

#[tokio::main]
async fn main() -> Result<()> {
    MqTrader::run().await?;
    Ok(())
}
