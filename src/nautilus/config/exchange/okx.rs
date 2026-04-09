use crate::nautilus::config::exchange::{ExchangeConfig, LiveExchangeClients, ProviderFilters};
use nautilus_live::config::{
    InstrumentProviderConfig, LiveDataClientConfig, LiveExecClientConfig, RoutingConfig,
};
use nautilus_model::identifiers::{AccountId, InstrumentId, Venue};
use nautilus_okx::OKXInstrumentType;
use nautilus_okx::common::enums::{OKXContractType, OKXMarginMode};
use nautilus_okx::config::{OKXDataClientConfig, OKXExecClientConfig};
use nautilus_okx::factories::{OKXDataClientFactory, OKXExecutionClientFactory};
use std::collections::HashMap;

// config/exchange/okx.rs
#[derive(Clone)]
pub struct OkxProviderFilters {
    pub instrument_types: Vec<OKXInstrumentType>,
    pub contract_types: Option<Vec<OKXContractType>>,
    pub instrument_families: Option<Vec<String>>,
}
impl OkxProviderFilters {
    /// 转换为通用 InstrumentProviderConfig
    /// `load_all` 由调用方传入，保持灵活性
    pub fn to_provider_config(&self, load_all: bool) -> InstrumentProviderConfig {
        let mut filters = HashMap::new();

        // instrument types 使用稳定字符串表示
        let types_str = self
            .instrument_types
            .iter()
            .map(|t| t.to_string()) // 需要 OKXInstrumentType impl Display
            .collect::<Vec<_>>()
            .join(",");
        filters.insert("instrument_types".to_string(), types_str);

        // contract types（可选）
        if let Some(cts) = &self.contract_types {
            let ct_str = cts
                .iter()
                .map(|c| c.to_string()) // 需要 OKXContractType impl Display
                .collect::<Vec<_>>()
                .join(",");
            filters.insert("contract_types".to_string(), ct_str);
        }

        // instrument families（可选）
        if let Some(fams) = &self.instrument_families {
            let fam_str = fams.join(",");
            filters.insert("instrument_families".to_string(), fam_str);
        }

        InstrumentProviderConfig {
            load_all,
            load_ids: false,
            filters,
        }
    }
}

pub struct OkxExchangeConfig {
    /// 显式指定 instruments（可选）
    pub instrument_ids: Option<Vec<InstrumentId>>,
    /// 账户ID
    pub account_id: AccountId,
    /// 是否加载全部
    pub load_all: bool,

    /// OKX 专用 filters
    pub filters: Option<OkxProviderFilters>,

    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub passphrase: Option<String>,
    pub is_demo: bool,
}

impl Default for OkxExchangeConfig {
    fn default() -> Self {
        Self {
            instrument_ids: None,
            account_id: AccountId::from("OKX-001"),
            load_all: true,
            filters: Some(OkxProviderFilters {
                instrument_types: vec![OKXInstrumentType::Futures],
                contract_types: Some(vec![OKXContractType::Linear]),
                instrument_families: Some(vec![
                    "BTC-USDT".to_string(),
                    "ETH-USDT".to_string(),
                    "SOL-USDT".to_string(),
                    "LINK-USDT".to_string(),
                    "XRP-USDT".to_string(),
                ]),
            }),
            api_key: None,
            api_secret: None,
            passphrase: None,
            is_demo: true,
        }
    }
}

impl OkxExchangeConfig {
    pub fn new(
        instrument_ids: Option<Vec<InstrumentId>>,
        load_all: bool,
        filters: Option<OkxProviderFilters>,
        api_key: Option<String>,
        api_secret: Option<String>,
        passphrase: Option<String>,
        is_demo: bool,
    ) -> Self {
        Self {
            instrument_ids,
            account_id: AccountId::from("OKX-001"),
            load_all,
            filters,
            api_key,
            api_secret,
            passphrase,
            is_demo,
        }
    }

    /// 构建默认 OKX 配置（示例/测试用）
    pub fn build_demo() -> Self {
        let api_key = Some("464e0adf-7f02-47ac-82d3-95736b15a114".to_string());
        let api_secret = Some("0D76268C7336050D755209E19AE7B5AA".to_string());
        let passphrase = Some("@8NautilusTrader".to_string());

        OkxExchangeConfig {
            account_id: AccountId::from("OKX-008"),
            instrument_ids: Some(vec![InstrumentId::from("ETH-USDT-SWAP.OKX")]),
            filters: Some(OkxProviderFilters {
                instrument_types: vec![OKXInstrumentType::Swap],
                contract_types: Some(vec![OKXContractType::Linear]),
                instrument_families: Some(vec![
                    "ETH-USDT".to_string(),
                    "SOL-USDT".to_string(),
                    "LINK-USDT".to_string(),
                    "XRP-USDT".to_string(),
                ]),
            }),
            api_key,
            api_secret,
            passphrase,
            is_demo: true,
            load_all: false,
            ..Default::default() // 其他字段保留 Default
        }
    }
}

impl ExchangeConfig for OkxExchangeConfig {
    fn venue(&self) -> Venue {
        Venue::from("OKX")
    }

    fn load_all_instruments(&self) -> bool {
        self.load_all
    }

    fn instrument_ids(&self) -> Option<Vec<InstrumentId>> {
        self.instrument_ids.clone()
    }

    fn provider_filters(&self) -> Option<ProviderFilters> {
        // Some(ProviderFilters::Okx(self.filters.clone()))
        self.filters.clone().map(ProviderFilters::Okx)
    }

    /// 构建 Live 配置（支持 runtime 注入）
    fn live_clients(&self) -> LiveExchangeClients {
        // 将 OKX Filters 转换为通用 InstrumentProviderConfig，并处理 None 的情况
        let instrument_provider = self
            .filters
            .as_ref()
            .map(|filters| filters.to_provider_config(self.load_all))
            .unwrap_or_default(); // 如果 filters 为 None，使用默认配置

        // 默认路由配置
        let routing = RoutingConfig {
            default: true,
            venues: Some(vec![self.venue().to_string()]),
        };

        // 创建 OKX 客户端工厂
        let data_client_factory = Box::new(OKXDataClientFactory::new());
        let exec_client_factory = Box::new(OKXExecutionClientFactory::new());

        let api_key = self.api_key.clone();
        let api_secret = self.api_secret.clone();
        let api_passphrase = self.passphrase.clone();
        let is_demo = self.is_demo; // 假设有一个获取 is_demo 的方法

        // 解包 Option 类型字段，使用默认值处理 None
        let instrument_types = self
            .filters
            .as_ref()
            .and_then(|filters| Some(filters.instrument_types.clone()))
            .unwrap_or_default();
        let contract_types = self
            .filters
            .as_ref()
            .and_then(|filters| filters.contract_types.clone())
            .unwrap_or_default();
        let instrument_families = self
            .filters
            .as_ref()
            .and_then(|filters| filters.instrument_families.clone())
            .unwrap_or_default();

        LiveExchangeClients {
            data: Some(LiveDataClientConfig {
                handle_revised_bars: false,
                instrument_provider: instrument_provider.clone(),
                routing: routing.clone(),
            }),
            exec: Some(LiveExecClientConfig {
                instrument_provider: instrument_provider.clone(),
                routing: routing.clone(),
            }),
            data_client_config: Some(Box::new(OKXDataClientConfig {
                api_key: api_key.clone(),
                api_secret: api_secret.clone(),
                api_passphrase: api_passphrase.clone(),
                instrument_types: instrument_types.clone(),
                contract_types: Some(contract_types.clone()),
                instrument_families: Some(instrument_families.clone()),
                // http_proxy_url: Some("http://localhost:8888".to_string()),
                // ws_proxy_url: Some("ws://localhost:8888".to_string()),
                http_proxy_url: None,
                ws_proxy_url: None,
                is_demo,
                ..Default::default()
            })),
            exec_client_config: Some(Box::new(OKXExecClientConfig {
                margin_mode: Some(OKXMarginMode::Cross),
                account_id: self.account_id.clone(),
                api_key,
                api_secret,
                api_passphrase,
                instrument_types,
                contract_types: Some(contract_types),
                instrument_families: Some(instrument_families),
                is_demo,
                // http_proxy_url: Some("http://localhost:8888".to_string()),
                // ws_proxy_url: Some("ws://localhost:8888".to_string()),
                http_proxy_url: None,
                ws_proxy_url: None,
                // 可选但推荐的配置
                use_fills_channel: false,      // VIP5+才可用
                use_mm_mass_cancel: false,     // 做市商批量撤单功能
                http_timeout_secs: 60u64,      // HTTP超时设置
                max_retries: 3u32,             // 最大重试次数
                retry_delay_initial_ms: 1_000, // 初始重试延迟
                retry_delay_max_ms: 10_000,    // 最大重试延迟
                ..Default::default()
            })),
            data_client_factory: Some(data_client_factory),
            exec_client_factory: Some(exec_client_factory),
        }
    }
}
