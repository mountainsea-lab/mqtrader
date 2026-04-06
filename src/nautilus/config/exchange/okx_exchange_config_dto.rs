use crate::nautilus::config::exchange::okx::{OkxExchangeConfig, OkxProviderFilters};
use nautilus_model::identifiers::{AccountId, InstrumentId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkxExchangeConfigDTO {
    pub account_id: String,
    pub instrument_ids: Option<Vec<String>>,
    pub filters: Option<OkxProviderFiltersDTO>,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub passphrase: Option<String>,
    pub is_demo: bool,
    pub load_all: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkxProviderFiltersDTO {
    pub instrument_types: Vec<String>, // 可以序列化成字符串
    pub contract_types: Option<Vec<String>>,
    pub instrument_families: Option<Vec<String>>,
}

impl From<&OkxExchangeConfig> for OkxExchangeConfigDTO {
    fn from(cfg: &OkxExchangeConfig) -> Self {
        Self {
            account_id: cfg.account_id.to_string(),
            instrument_ids: cfg
                .instrument_ids
                .as_ref()
                .map(|v| v.iter().map(|id| id.to_string()).collect()),
            filters: cfg.filters.as_ref().map(|f| OkxProviderFiltersDTO {
                instrument_types: f
                    .instrument_types
                    .iter()
                    .map(|t| format!("{:?}", t))
                    .collect(),
                contract_types: f
                    .contract_types
                    .as_ref()
                    .map(|c| c.iter().map(|c| format!("{:?}", c)).collect()),
                instrument_families: f.instrument_families.clone(),
            }),
            api_key: cfg.api_key.clone(),
            api_secret: cfg.api_secret.clone(),
            passphrase: cfg.passphrase.clone(),
            is_demo: cfg.is_demo,
            load_all: cfg.load_all,
        }
    }
}

impl TryFrom<OkxExchangeConfigDTO> for OkxExchangeConfig {
    type Error = anyhow::Error;

    fn try_from(dto: OkxExchangeConfigDTO) -> anyhow::Result<Self> {
        Ok(Self {
            account_id: AccountId::from(dto.account_id),
            instrument_ids: dto
                .instrument_ids
                .map(|v| v.into_iter().map(InstrumentId::from).collect()),
            filters: dto.filters.map(|f| OkxProviderFilters {
                instrument_types: f
                    .instrument_types
                    .into_iter()
                    .map(|t| t.parse().unwrap())
                    .collect(),
                contract_types: f
                    .contract_types
                    .map(|c| c.into_iter().map(|c| c.parse().unwrap()).collect()),
                instrument_families: f.instrument_families,
            }),
            api_key: dto.api_key,
            api_secret: dto.api_secret,
            passphrase: dto.passphrase,
            is_demo: dto.is_demo,
            load_all: dto.load_all,
            ..Default::default()
        })
    }
}
