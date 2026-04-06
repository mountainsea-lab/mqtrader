use crate::nautilus::config::exchange::ExchangeConfigDTO;
use crate::nautilus::config::node::logger_config_dto::LoggerConfigDTO;
use anyhow::Result;
use dynwrap_strategy::SConfigSerializable;
use nautilus_common::cache::CacheConfig;
use nautilus_common::enums::Environment;
use nautilus_common::msgbus::database::MessageBusConfig;
use nautilus_core::UUID4;
use nautilus_live::config::{
    LiveDataClientConfig, LiveDataEngineConfig, LiveExecClientConfig, LiveExecEngineConfig,
    LiveNodeConfig, LiveRiskEngineConfig,
};
use nautilus_model::identifiers::TraderId;
use nautilus_portfolio::config::PortfolioConfig;
use nautilus_system::StreamingConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfigDTO {
    pub environment: String,
    pub trader_id: String,

    pub load_state: bool,
    pub save_state: bool,

    pub instance_id: Option<String>,

    pub timeout_connection_secs: u64,
    pub timeout_reconciliation_secs: u64,
    pub timeout_portfolio_secs: u64,
    pub timeout_disconnection_secs: u64,
    pub delay_post_stop_secs: u64,
    pub timeout_shutdown_secs: u64,

    pub exchanges: HashMap<String, ExchangeConfigDTO>, // key: "OKX", "BINANCE"…

    pub logging: LoggerConfigDTO,
    pub cache: Option<CacheConfig>,
    pub msgbus: Option<MessageBusConfig>,
    pub portfolio: Option<PortfolioConfig>,

    pub data_engine: LiveDataEngineConfig,
    pub risk_engine: LiveRiskEngineConfig,
    pub exec_engine: LiveExecEngineConfig,

    pub data_clients: HashMap<String, LiveDataClientConfig>,
    pub exec_clients: HashMap<String, LiveExecClientConfig>,
}

impl TryFrom<NodeConfigDTO> for LiveNodeConfig {
    type Error = anyhow::Error;

    fn try_from(dto: NodeConfigDTO) -> Result<Self> {
        Ok(Self {
            environment: match dto.environment.as_str() {
                "Live" => Environment::Live,
                _ => anyhow::bail!("Invalid environment"),
            },

            trader_id: TraderId::from(dto.trader_id),

            load_state: dto.load_state,
            save_state: dto.save_state,

            logging: dto.logging.try_into()?,
            instance_id: match dto.instance_id {
                Some(id) => Some(UUID4::from(id.as_str())), // 如果库没 Result 只能这样
                None => None,
            },

            timeout_connection: Duration::from_secs(dto.timeout_connection_secs),
            timeout_reconciliation: Duration::from_secs(dto.timeout_reconciliation_secs),
            timeout_portfolio: Duration::from_secs(dto.timeout_portfolio_secs),
            timeout_disconnection: Duration::from_secs(dto.timeout_disconnection_secs),
            delay_post_stop: Duration::from_secs(dto.delay_post_stop_secs),
            timeout_shutdown: Duration::from_secs(dto.timeout_shutdown_secs),

            cache: dto.cache,
            msgbus: dto.msgbus,
            portfolio: dto.portfolio,

            streaming: Default::default(),
            data_engine: dto.data_engine,
            risk_engine: dto.risk_engine,
            exec_engine: dto.exec_engine,

            data_clients: dto.data_clients,
            exec_clients: dto.exec_clients,
        })
    }
}

impl From<&LiveNodeConfig> for NodeConfigDTO {
    fn from(cfg: &LiveNodeConfig) -> Self {
        Self {
            environment: format!("{:?}", cfg.environment),
            trader_id: cfg.trader_id.to_string(),

            load_state: cfg.load_state,
            save_state: cfg.save_state,

            instance_id: cfg.instance_id.map(|id| id.to_string()),

            timeout_connection_secs: cfg.timeout_connection.as_secs(),
            timeout_reconciliation_secs: cfg.timeout_reconciliation.as_secs(),
            timeout_portfolio_secs: cfg.timeout_portfolio.as_secs(),
            timeout_disconnection_secs: cfg.timeout_disconnection.as_secs(),
            delay_post_stop_secs: cfg.delay_post_stop.as_secs(),
            timeout_shutdown_secs: cfg.timeout_shutdown.as_secs(),

            exchanges: HashMap::new(),
            logging: (&cfg.logging).into(),
            cache: cfg.cache.clone(),
            msgbus: cfg.msgbus.clone(),
            portfolio: cfg.portfolio.clone(),

            data_engine: cfg.data_engine.clone(),
            risk_engine: cfg.risk_engine.clone(),
            exec_engine: cfg.exec_engine.clone(),

            data_clients: cfg.data_clients.clone(),
            exec_clients: cfg.exec_clients.clone(),
        }
    }
}
impl SConfigSerializable for NodeConfigDTO {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nautilus::config::exchange::okx::OkxExchangeConfig;
    use crate::nautilus::config::exchange::okx_exchange_config_dto::OkxExchangeConfigDTO;
    use crate::nautilus::service::trading_service::TradingService;
    #[test]
    fn test_node_config_file_roundtrip() -> Result<()> {
        use std::fs;
        use std::path::Path;

        // 1️⃣ 构建默认配置
        let original = TradingService::build_node_config()?;

        // 2️⃣ 转 DTO
        let mut dto = NodeConfigDTO::from(&original);

        let mut exchanges = HashMap::new();
        let okx_config = OkxExchangeConfig::build_demo();
        let okx_dto: OkxExchangeConfigDTO = (&okx_config).into();
        exchanges.insert("OKX".to_string(), ExchangeConfigDTO::Okx(okx_dto));
        dto.exchanges = exchanges;

        // 3️⃣ 写入 config 目录
        let config_dir = Path::new("config");
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
        }

        let file_path = config_dir.join("node_config.json");

        dto.write_to_file(&file_path)?;

        // 4️⃣ 从文件读取 DTO
        let dto2 = NodeConfigDTO::from_file(&file_path)?;
        // 5️⃣ 转回 LiveNodeConfig
        let rebuilt = LiveNodeConfig::try_from(dto2)?;

        // 6️⃣ 核心字段断言
        assert_eq!(original.trader_id, rebuilt.trader_id);
        assert_eq!(original.load_state, rebuilt.load_state);
        assert_eq!(original.save_state, rebuilt.save_state);

        assert_eq!(
            original.timeout_connection.as_secs(),
            rebuilt.timeout_connection.as_secs()
        );

        assert_eq!(
            original.timeout_reconciliation.as_secs(),
            rebuilt.timeout_reconciliation.as_secs()
        );

        // 7️⃣ 清理文件（重要！避免污染）
        // fs::remove_file(file_path)?;

        Ok(())
    }

    #[test]
    fn test_load_from_json_file_temp() -> Result<()> {
        use tempfile::tempdir;

        // 临时目录
        let dir = tempdir()?;
        let file_path = dir.path().join("node.json");

        // 1️⃣ 构建配置
        let original = TradingService::build_node_config()?;
        let mut dto = NodeConfigDTO::from(&original);

        // 2️⃣ 补充交易所配置
        let mut exchanges = HashMap::new();
        let okx_config = OkxExchangeConfig::build_demo();
        let okx_dto: OkxExchangeConfigDTO = (&okx_config).into();
        exchanges.insert("OKX".to_string(), ExchangeConfigDTO::Okx(okx_dto));
        dto.exchanges = exchanges;

        // 3️⃣ 写入
        dto.write_to_file(&file_path)?;

        // 4️⃣ 读取
        let dto_loaded = NodeConfigDTO::from_file(&file_path)?;
        let dto_loaded_clone = dto_loaded.clone();
        // 5️⃣ 转换
        let config = LiveNodeConfig::try_from(dto_loaded)?;

        // 6️⃣ 断言
        assert_eq!(config.trader_id, original.trader_id);
        assert_eq!(config.load_state, original.load_state);

        // 7️⃣ 交易所字段断言
        if let Some(ExchangeConfigDTO::Okx(okx)) = dto_loaded_clone.exchanges.get("OKX") {
            // assert_eq!(okx.account_id, okx_config.account_id);
            assert_eq!(okx.load_all, okx_config.load_all);
            assert_eq!(okx.is_demo, okx_config.is_demo);
            // assert_eq!(okx.instrument_ids, okx_config.instrument_ids);
            assert_eq!(
                okx.filters.as_ref().unwrap().instrument_families,
                okx_config.filters.as_ref().unwrap().instrument_families
            );
        } else {
            panic!("OKX exchange config missing in loaded NodeConfig");
        }
        Ok(())
    }
}
