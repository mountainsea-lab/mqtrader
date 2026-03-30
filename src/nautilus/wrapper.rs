pub mod live;
use nautilus_common::actor::Component;
use nautilus_live::node::LiveNode;
use nautilus_trading::Strategy;
use std::fmt::Debug;

pub enum NautilusWrapper {
    Live(LiveNode),
}

impl NautilusWrapper {
    pub async fn run(&mut self) -> anyhow::Result<()> {
        match self {
            Self::Live(node) => node.run().await,
        }
    }

    pub async fn stop(&mut self) -> anyhow::Result<()> {
        match self {
            Self::Live(node) => node.stop().await,
        }
    }

    pub fn add_strategy<T>(&mut self, strategy: T) -> anyhow::Result<()>
    where
        T: Strategy + Component + Debug + 'static,
    {
        match self {
            Self::Live(node) => node.add_strategy(strategy),
        }
    }

    pub fn add_strategies<I, T>(&mut self, strategies: I) -> anyhow::Result<()>
    where
        I: IntoIterator<Item = T>,
        T: Strategy + Component + Debug + 'static,
    {
        match self {
            Self::Live(node) => strategies
                .into_iter()
                .try_for_each(|s| node.add_strategy(s)), // NautilusKernelWrapper::Backtest(engine) => {
        }
    }
}
