use anyhow::Result;
use mqtrader::nautilus::service::trading_service::TradingService;

#[tokio::main]
async fn main() -> Result<()> {
    TradingService::run(true).await?;
    Ok(())
}
