use crate::nautilus::config::exchange::ExchangeConfig;
use crate::nautilus::wrapper::NautilusWrapper;
use crate::nautilus::wrapper::live::LiveNodeWrapper;
use nautilus_common::component::Component;
use nautilus_live::config::LiveNodeConfig;
use nautilus_trading::Strategy;
use std::fmt::Debug;

/// 单节点 TradingKernel
pub struct TradingKernel {
    node_wrapper: NautilusWrapper,
}

impl TradingKernel {
    pub async fn live(
        config: LiveNodeConfig,
        exchange: Box<dyn ExchangeConfig>,
    ) -> anyhow::Result<Self> {
        let live_node = LiveNodeWrapper::build_live_node(config.clone(), &exchange).await?;
        let node_wrapper = NautilusWrapper::Live(live_node);

        Ok(Self { node_wrapper })
    }
}

impl TradingKernel {
    pub fn add_strategy<T>(&mut self, strategy: T) -> anyhow::Result<()>
    where
        T: Strategy + Component + Debug + 'static,
    {
        self.node_wrapper.add_strategy(strategy)
    }

    pub fn add_strategies<I, T>(&mut self, strategies: I) -> anyhow::Result<()>
    where
        I: IntoIterator<Item = T>,
        T: Strategy + Component + Debug + 'static,
    {
        self.node_wrapper.add_strategies(strategies)
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.node_wrapper.run().await?;

        Ok(())
    }

    pub async fn stop(&mut self) -> anyhow::Result<()> {
        self.node_wrapper.stop().await
    }
}
