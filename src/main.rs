use anyhow::Result;
use mqtrader::nautilus::bot::mqtrader::MqTrader;
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> Result<()> {
    MqTrader::run().await?;
    Ok(())
}
