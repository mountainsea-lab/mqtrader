use crate::nautilus::config::node::logger_config_dto::LoggerConfigDTO;
use nautilus_common::enums::Environment;
use nautilus_live::config::LiveNodeConfig;
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
    // pub logging: LoggerConfigDTO,
    // pub cache: Option<CacheConfig>,
    // pub msgbus: Option<MessageBusConfig>,
    // pub portfolio: Option<PortfolioConfig>,
    // pub streaming: Option<StreamingConfig>,
    //
    // pub data_engine: LiveDataEngineConfig,
    // pub risk_engine: LiveRiskEngineConfig,
    // pub exec_engine: LiveExecEngineConfig,
    //
    // pub data_clients: HashMap<String, LiveDataClientConfig>,
    // pub exec_clients: HashMap<String, LiveExecClientConfig>,
}
//
// impl TryFrom<NodeConfigDTO> for LiveNodeConfig {
//     type Error = anyhow::Error;
//
//     fn try_from(dto: NodeConfigDTO) -> Result<Self> {
//         Ok(Self {
//             environment: match dto.environment.as_str() {
//                 "Live" => Environment::Live,
//                 _ => anyhow::bail!("Invalid environment"),
//             },
//
//             trader_id: TraderId::from(dto.trader_id),
//
//             load_state: dto.load_state,
//             save_state: dto.save_state,
//
//             instance_id: dto
//                 .instance_id
//                 .map(|id| UUID4::from(id.as_str())),
//
//             timeout_connection: Duration::from_secs(dto.timeout_connection_secs),
//             timeout_reconciliation: Duration::from_secs(dto.timeout_reconciliation_secs),
//             timeout_portfolio: Duration::from_secs(dto.timeout_portfolio_secs),
//             timeout_disconnection: Duration::from_secs(dto.timeout_disconnection_secs),
//             delay_post_stop: Duration::from_secs(dto.delay_post_stop_secs),
//             timeout_shutdown: Duration::from_secs(dto.timeout_shutdown_secs),
//
//
//             // cache: dto.cache,
//             // msgbus: dto.msgbus,
//             // portfolio: dto.portfolio,
//             // streaming: dto.streaming,
//             //
//             // data_engine: dto.data_engine,
//             // risk_engine: dto.risk_engine,
//             // exec_engine: dto.exec_engine,
//             //
//             // data_clients: dto.data_clients,
//             // exec_clients: dto.exec_clients,
//             exec_clients: (),
//         })
//     }
// }
//
// impl From<&LiveNodeConfig> for NodeConfigDTO {
//     fn from(cfg: &LiveNodeConfig) -> Self {
//         Self {
//             environment: format!("{:?}", cfg.environment),
//             trader_id: cfg.trader_id.to_string(),
//
//             load_state: cfg.load_state,
//             save_state: cfg.save_state,
//
//             instance_id: cfg.instance_id.map(|id| id.to_string()),
//
//             timeout_connection_secs: cfg.timeout_connection.as_secs(),
//             timeout_reconciliation_secs: cfg.timeout_reconciliation.as_secs(),
//             timeout_portfolio_secs: cfg.timeout_portfolio.as_secs(),
//             timeout_disconnection_secs: cfg.timeout_disconnection.as_secs(),
//             delay_post_stop_secs: cfg.delay_post_stop.as_secs(),
//             timeout_shutdown_secs: cfg.timeout_shutdown.as_secs(),
//
//             logging: cfg.logging.clone(),
//             cache: cfg.cache.clone(),
//             msgbus: cfg.msgbus.clone(),
//             portfolio: cfg.portfolio.clone(),
//             streaming: cfg.streaming.clone(),
//
//             data_engine: cfg.data_engine.clone(),
//             risk_engine: cfg.risk_engine.clone(),
//             exec_engine: cfg.exec_engine.clone(),
//
//             data_clients: cfg.data_clients.clone(),
//             exec_clients: cfg.exec_clients.clone(),
//         }
//     }
// }
