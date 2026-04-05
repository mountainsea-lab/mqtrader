use anyhow::{Context, Result};
use dynwrap_strategy::StrategyExt;
use libloading::Library;
use log::info;
use nautilus_trading::Strategy;
use std::env;

pub fn load_strategy() -> Result<Box<dyn StrategyExt>> {
    // 获取环境变量
    let lib_path = get_strategy_lib_path();
    info!("Loading strategy lib_path...{}", lib_path);
    let config_path =
        env::var("STRATEGY_CONFIG").context("Missing STRATEGY_CONFIG environment variable")?;
    // 加载动态库
    let library = unsafe { Library::new(lib_path).context("Failed to load dynamic library") }?;
    pub type create_strategy = fn(config_path: &str) -> Result<Box<dyn StrategyExt>>;
    let func: create_strategy = *unsafe { library.get("create_strategy".as_bytes()) }?;
    let strategy = func(&config_path)?;
    // 返回 Box<dyn Strategy>
    Ok(strategy)
}

fn get_strategy_lib_path() -> String {
    let base_path = env::var("STRATEGY_LIB").expect("STRATEGY_LIB not set");

    #[cfg(target_os = "linux")]
    return format!("{}.so", base_path);

    #[cfg(target_os = "macos")]
    return format!("{}.dylib", base_path);

    #[cfg(target_os = "windows")]
    return format!("{}.dll", base_path);
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_strategy() {
        // 假设动态库和配置文件路径正确
        match load_strategy() {
            Ok(strategy) => {
                // 检查策略实例是否有效
                assert!(strategy.s_config().base().external_order_claims.is_some());
            }
            Err(e) => {
                panic!("Failed to load strategy: {:?}", e);
            }
        }
    }
}
