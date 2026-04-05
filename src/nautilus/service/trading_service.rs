use crate::nautilus::config::exchange::okx::{OkxExchangeConfig, OkxProviderFilters};
use crate::nautilus::trading_kernel::TradingKernel;
use ahash::AHashMap;
use anyhow::{Context, Result};
use dynwrap_strategy::strategy_wrapper_ffi::DynStrategyWrapper;
use log::{LevelFilter, info};
use nautilus_common::cache::CacheConfig;
use nautilus_common::enums::{Environment, SerializationEncoding};
use nautilus_common::logging::logger::LoggerConfig;
use nautilus_common::msgbus::database::{DatabaseConfig, MessageBusConfig};
use nautilus_core::UUID4;
use nautilus_live::config::{
    LiveDataEngineConfig, LiveExecEngineConfig, LiveNodeConfig, LiveRiskEngineConfig,
};
use nautilus_model::identifiers::{AccountId, InstrumentId, TraderId};
use nautilus_okx::OKXInstrumentType;
use nautilus_okx::common::enums::OKXContractType;
use nautilus_portfolio::config::PortfolioConfig;
use std::env;
use std::path::Path;
use ustr::Ustr;

pub struct TradingService;

impl TradingService {
    // -----------------------------
    // 异步执行 Sandbox / Live
    // -----------------------------
    pub async fn run(is_demo: bool) -> Result<()> {
        // 构建 async 内核
        let mut launcher = TradingService::build_kernel(is_demo).await?;
        // 加入策略
        TradingService::load_strategies(&mut launcher)?;
        launcher.run().await?;

        Ok(())
    }

    async fn build_kernel(is_demo: bool) -> Result<TradingKernel> {
        let okx_config = Self::build_exchange_config(is_demo);
        let node_config = Self::build_node_config()?;
        let launcher = TradingKernel::live(node_config, Box::new(okx_config)).await?;

        Ok(launcher)
    }

    pub fn build_node_config() -> Result<LiveNodeConfig> {
        let instance_id = UUID4::new();
        let trader_id = TraderId::from("OKX-TRADER-001");

        let logging_config = Self::build_logging_config()?;
        let cache_config = Self::build_cache_config();
        let msgbus_config = Self::build_msgbus_config();
        let exec_engine_config = Self::build_exec_engine_config();

        let node_config = LiveNodeConfig {
            trader_id,
            environment: Environment::Live,
            instance_id: Some(instance_id),

            load_state: false,
            save_state: false,

            timeout_connection: std::time::Duration::from_secs(60),
            timeout_reconciliation: std::time::Duration::from_secs(10),
            timeout_portfolio: std::time::Duration::from_secs(60),
            timeout_disconnection: std::time::Duration::from_secs(10),

            delay_post_stop: Default::default(),
            timeout_shutdown: Default::default(),

            cache: Some(cache_config),
            msgbus: Some(msgbus_config),

            portfolio: Some(PortfolioConfig {
                min_account_state_logging_interval_ms: Some(300000),
                ..Default::default()
            }),
            streaming: Default::default(),
            data_engine: LiveDataEngineConfig { qsize: 10_000 },
            risk_engine: LiveRiskEngineConfig { qsize: 10_000 },
            exec_engine: exec_engine_config,

            data_clients: Default::default(),
            exec_clients: Default::default(),

            logging: logging_config,
        };

        Ok(node_config)
    }

    /// 获取策略动态库路径（自动适配平台扩展名）
    fn get_strategy_lib_path() -> Result<String> {
        let base_path =
            env::var("STRATEGY_LIB").context("STRATEGY_LIB environment variable not set")?;

        let path = Path::new(&base_path);

        // 如果已有扩展名，直接返回
        if path.extension().is_some() {
            return Ok(base_path);
        }

        // 否则添加平台特定扩展名
        let mut path_buf = path.to_path_buf();
        #[cfg(target_os = "linux")]
        path_buf.set_extension("so");
        #[cfg(target_os = "macos")]
        path_buf.set_extension("dylib");
        #[cfg(target_os = "windows")]
        path_buf.set_extension("dll");

        path_buf
            .to_str()
            .map(|s| s.to_string())
            .context("Failed to convert path to string")
    }

    /// 加载策略插件
    pub fn load_strategies(launcher: &mut TradingKernel) -> Result<()> {
        // 获取动态库路径
        let lib_path = Self::get_strategy_lib_path()?;
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

    // // 批量构建交易资产策略
    // fn build_batch_strategies(
    //     assets: Vec<&str>,
    //     time_frame: &str,
    //     assets_fomat: &str,
    // ) -> Vec<GmmasrStrategy> {
    //     // 基于传入的资产列表和时间框架批量构建策略
    //     assets
    //         .into_iter()
    //         .map(|asset| {
    //             let symbol = format!("{}-{}", asset, assets_fomat); // 将资产和 USDT 拼接成完整的交易对 symbol
    //             GmmasrStrategy::build_strategy(&symbol, time_frame, asset)
    //         })
    //         .collect()
    // }

    fn build_logging_config() -> Result<LoggerConfig> {
        let config = LoggerConfig {
            stdout_level: LevelFilter::Info,
            fileout_level: LevelFilter::Off,  // 禁用文件输出
            component_level: AHashMap::new(), // 不使用组件级过滤
            module_level: AHashMap::from([
                // 匹配你的GMMA策略模块
                (
                    Ustr::from("strategy::gmmasr::gmmasr_strategy"),
                    LevelFilter::Info,
                ),
                // 订单相关模块（如果需要）
                (Ustr::from("nautilus_execution::"), LevelFilter::Info),
            ]),
            log_components_only: false, // 使用模块过滤时设为false
            is_colored: false,
            print_config: false,
            use_tracing: false,
        };
        Ok(config)
    }

    fn build_exchange_config(is_demo: bool) -> OkxExchangeConfig {
        // let mut is_demo = false;
        // let mut api_key = Some("87b2322a-b0dd-4a4b-84de-e3b23cf53fe4".to_string());
        // let mut api_secret = Some("DE6D29E2CA70E8C854D45D1E883BEBE9".to_string());
        // let mut passphrase = Some("@Zt2307631397".to_string());

        // if environment == Environment::Live {
        // 	is_demo = false;
        // 	api_key = Some("464e0adf-7f02-47ac-82d3-95736b15a114".to_string());
        // 	api_secret = Some("0D76268C7336050D755209E19AE7B5AA".to_string());
        // 	passphrase = Some("@8NautilusTrader".to_string());
        // }

        let api_key = Some("464e0adf-7f02-47ac-82d3-95736b15a114".to_string());
        let api_secret = Some("0D76268C7336050D755209E19AE7B5AA".to_string());
        let passphrase = Some("@8NautilusTrader".to_string());

        OkxExchangeConfig {
            account_id: AccountId::from("OKX-008"),

            instrument_ids: Some(vec![InstrumentId::from("ETH-USDT-SWAP.OKX")]),

            filters: Some(OkxProviderFilters {
                instrument_types: vec![OKXInstrumentType::Swap],
                contract_types: Some(vec![OKXContractType::Linear]),
                /// 注意如果需要新增交易资产这里必须指定
                ///  "BTC","SOL","ETH", "LINK", "ADA", "XRP", "BNB", "AVAX", "DOGE", "ETC", "AAVE"
                instrument_families: Some(vec![
                    // "BTC-USDT".to_string(),
                    "ETH-USDT".to_string(),
                    "SOL-USDT".to_string(),
                    "LINK-USDT".to_string(),
                    // "ADA-USDT".to_string(),
                    "XRP-USDT".to_string(),
                    // "BNB-USDT".to_string(),
                    // "AVAX-USDT".to_string(),
                    // "DOGE-USDT".to_string(),
                    // "ETC-USDT".to_string(),
                    // "AAVE-USDT".to_string(),
                ]),
            }),
            api_key,
            api_secret,
            passphrase,
            is_demo,
            load_all: false,
            ..Default::default()
        }
    }

    fn build_cache_config() -> CacheConfig {
        // todo 暂时不配置数据库
        // let pg = Self::build_pg_config();
        let data_base = Self::build_redis_config();
        // - 按资产数量调整
        CacheConfig {
            database: Some(data_base),
            encoding: SerializationEncoding::MsgPack,
            timestamps_as_iso8601: false,
            buffer_interval_ms: Some(100),
            bulk_read_batch_size: Some(50),
            use_trader_prefix: true,
            use_instance_id: true,
            flush_on_start: true,
            drop_instruments_on_reset: true, // 重置时清理合约数据
            tick_capacity: 6,
            bar_capacity: 1000,
            save_market_data: false, // 不持久化市场数据
        }
    }

    fn build_msgbus_config() -> MessageBusConfig {
        // let data_base = Self::build_pg_config();
        let data_base = Self::build_redis_config();

        MessageBusConfig {
            database: Some(data_base),
            encoding: SerializationEncoding::MsgPack,
            timestamps_as_iso8601: false,
            buffer_interval_ms: Some(100),

            autotrim_mins: Some(1440),

            use_trader_prefix: true,
            use_trader_id: true,
            use_instance_id: true,

            streams_prefix: "nautilus".to_string(),
            stream_per_topic: true,

            external_streams: None,

            types_filter: Some(vec!["QuoteTick".to_string(), "TradeTick".to_string()]),

            heartbeat_interval_secs: Some(1),
        }
    }
    fn build_redis_config() -> DatabaseConfig {
        DatabaseConfig {
            database_type: "redis".to_string(),
            host: Some("localhost".to_string()),
            port: Some(6379),
            username: None,
            password: Some("Cdz@2024".to_string()),
            // 使用合理的超时和重试值
            connection_timeout: 20,
            response_timeout: 20,
            number_of_retries: 100,
            exponent_base: 2,
            max_delay: 1000,
            factor: 2,
            ..Default::default() // 其他字段使用默认值
        }
    }
    fn build_pg_config() -> DatabaseConfig {
        DatabaseConfig {
            database_type: "postgres".to_string(),
            host: Some("192.168.10.4".to_string()), // 线上 43.157.43.100
            port: Some(5432),
            username: Some("root".to_string()),
            password: Some("root".to_string()),
            ssl: false,
            connection_timeout: 20,
            response_timeout: 20,
            number_of_retries: 3,
            exponent_base: 2,
            max_delay: 10,
            factor: 2,
        }
    }

    fn build_exec_engine_config() -> LiveExecEngineConfig {
        LiveExecEngineConfig {
            reconciliation: true,
            //  内存清理 - 10资产场景优化
            purge_closed_orders_interval_mins: Some(15), // 8分钟清理一次
            purge_closed_orders_buffer_mins: Some(60),   // 保留45分钟
            purge_closed_positions_interval_mins: Some(15), // 8分钟清理一次
            purge_closed_positions_buffer_mins: Some(60), // 保留45分钟
            purge_account_events_interval_mins: Some(15), // 8分钟清理一次
            purge_account_events_lookback_mins: Some(60), // 保留45分钟
            purge_from_database: false,                  // 保留数据库记录
            reconciliation_startup_delay_secs: 5.0,
            reconciliation_lookback_mins: Some(60),

            reconciliation_instrument_ids: None,

            filter_unclaimed_external_orders: true,
            filter_position_reports: false,

            filtered_client_order_ids: None,

            generate_missing_orders: true,

            inflight_check_interval_ms: 2000,
            inflight_check_threshold_ms: 10000,
            inflight_check_retries: 3,

            open_check_interval_secs: Some(30.0),
            open_check_lookback_mins: Some(10),
            open_check_threshold_ms: 5000,
            open_check_missing_retries: 3,
            open_check_open_only: true,

            max_single_order_queries_per_cycle: 10,
            single_order_query_delay_ms: 100,

            position_check_interval_secs: Some(60.0),
            position_check_lookback_mins: 30,
            position_check_threshold_ms: 5000,
            position_check_retries: 0,
            own_books_audit_interval_secs: Some(3600.0),
            graceful_shutdown_on_error: true,

            qsize: 10_000,
        }
    }
}
