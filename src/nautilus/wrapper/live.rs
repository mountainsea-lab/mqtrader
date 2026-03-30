use crate::nautilus::config::exchange::ExchangeConfig;
use nautilus_data::DataClientAdapter;
use nautilus_live::config::LiveNodeConfig;
use nautilus_live::node::LiveNode;

#[derive(Debug)]
pub struct LiveNodeWrapper;

impl LiveNodeWrapper {
    pub async fn build_live_node(
        config: LiveNodeConfig,
        exchange: &Box<dyn ExchangeConfig>,
    ) -> anyhow::Result<LiveNode> {
        // 使用完整配置创建node
        let node = LiveNode::build(format!("{}-node", exchange.venue()), Some(config))?;

        // 手动创建和注册客户端
        let mut live_clients = exchange.live_clients();

        if let (Some(factory), Some(cfg)) = (
            live_clients.data_client_factory.take(),
            live_clients.data_client_config.take(),
        ) {
            let client = factory.create(
                &exchange.venue().as_str(),
                cfg.as_ref(),
                node.kernel().cache(),
                node.kernel().clock(),
            )?;

            // 直接注册到kernel的data_engine
            let adapter =
                DataClientAdapter::new(client.client_id(), client.venue(), true, true, client);

            node.kernel()
                .data_engine
                .borrow_mut()
                .register_client(adapter, Some(exchange.venue()));
        }

        // 类似处理execution client
        if let (Some(factory), Some(cfg)) = (
            live_clients.exec_client_factory.take(),
            live_clients.exec_client_config.take(),
        ) {
            let client = factory.create(
                &exchange.venue().as_str(),
                cfg.as_ref(),
                node.kernel().cache(),
            )?;
            node.kernel()
                .exec_engine
                .borrow_mut()
                .register_client(client)?;
        }

        Ok(node)
    }
}
