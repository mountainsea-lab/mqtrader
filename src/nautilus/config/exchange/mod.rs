// config/exchange/exchange.rs

use crate::nautilus::config::exchange::okx::OkxProviderFilters;
use nautilus_live::config::{LiveDataClientConfig, LiveExecClientConfig};
use nautilus_model::identifiers::{InstrumentId, Venue};
use nautilus_system::{ClientConfig, DataClientFactory, ExecutionClientFactory};
use serde_json::Value;
use std::collections::HashMap;

pub struct LiveExchangeClients {
    /// 通用配置
    pub data: Option<LiveDataClientConfig>,
    pub exec: Option<LiveExecClientConfig>,

    /// 客户端配置
    pub data_client_config: Option<Box<dyn ClientConfig>>,
    pub exec_client_config: Option<Box<dyn ClientConfig>>,

    /// 客户端工厂
    pub data_client_factory: Option<Box<dyn DataClientFactory>>,
    pub exec_client_factory: Option<Box<dyn ExecutionClientFactory>>,
}

#[derive(Clone)]
pub enum ProviderFilters {
    Okx(OkxProviderFilters),
    // Bybit(BybitInstrumentFilters),
    Tardis(HashMap<String, Value>),
}

pub trait ExchangeConfig {
    /// 交易所标识
    fn venue(&self) -> Venue;

    /// 是否加载全部 Instrument
    fn load_all_instruments(&self) -> bool;

    /// 指定要加载的 Instrument IDs（可选）
    fn instrument_ids(&self) -> Option<Vec<InstrumentId>>;

    /// Adapter-specific filters（透传）
    fn provider_filters(&self) -> Option<ProviderFilters>;

    /// 构建 Live 所需的 client configs
    fn live_clients(&self) -> LiveExchangeClients;
}

pub mod okx;
mod okx_exchange_config_dto;
