use crate::nautilus::bot::get_strategy_lib_path;
use crate::nautilus::config::exchange::ExchangeConfig;
use crate::nautilus::config::node::node_config_dto::NodeConfigDTO;
use crate::nautilus::trading_kernel::TradingKernel;
use anyhow::{Context, Result};
use dynwrap_strategy::SConfigSerializable;
use dynwrap_strategy::strategy_wrapper_ffi::DynStrategyWrapper;
use log::info;
use std::env;
use std::path::Path;

pub struct MqTrader;

impl MqTrader {
    // -----------------------------
    // 异步执行 Sandbox / Live
    // -----------------------------
    pub async fn run() -> Result<()> {
        // 构建 async 内核
        let mut launcher = Self::build_kernel().await?;
        // 加入策略
        Self::load_strategies(&mut launcher)?;
        launcher.run().await?;

        Ok(())
    }

    async fn build_kernel() -> Result<TradingKernel> {
        let config_path = env::var("TRADER_NODE_CONFIG")?;
        info!("Using TRADER_NODE_CONFIG: {}", config_path);

        let node_config_dto = NodeConfigDTO::from_file(&config_path)?;
        let (node_config, mut live_exchanges) = node_config_dto.into_node_config()?;
        //  默认取OKX交易所配置（确保 JSON 里有 "OKX"）
        let okx_config: Box<dyn ExchangeConfig> = live_exchanges
            .remove("OKX")
            .context("OKX exchange config not found in NodeConfigDTO")?;
        let launcher = TradingKernel::live(node_config, okx_config).await?;

        Ok(launcher)
    }

    /// 加载策略插件
    pub fn load_strategies(launcher: &mut TradingKernel) -> Result<()> {
        // 获取动态库路径
        let lib_path = get_strategy_lib_path()?;
        info!("Loading strategy from: {}", lib_path);

        // 获取配置路径
        let config_path =
            env::var("STRATEGY_CONFIG").context("STRATEGY_CONFIG environment variable not set")?;
        info!("Using config: {}", config_path);

        // 加载动态库策略
        let strategy = DynStrategyWrapper::load(Path::new(&lib_path), &config_path)
            .context(format!("Failed to load strategy from {}", lib_path))?;

        launcher.add_strategy(strategy)?;
        info!("Strategy loaded successfully");

        Ok(())
    }
}
