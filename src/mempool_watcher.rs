use ethers::prelude::*;
use futures_util::StreamExt;
use std::sync::Arc;
use crate::log_info;

pub struct MempoolWatcher<M>
where
    M: Middleware + 'static,
{
    pub provider: Arc<M>,
    pub pengu_pair_address: Address,
}

impl<M> MempoolWatcher<M>
where
    M: Middleware + 'static,
{
    pub fn new(provider: Arc<M>, pengu_pair_address: Address) -> Self {
        Self { provider, pengu_pair_address }
    }

    pub async fn watch_pending(&self) -> anyhow::Result<()>
    where <M as Middleware>::Provider: PubsubClient {
        log_info!("MempoolWatcher watching: {:?}", self.pengu_pair_address);

        let mut stream = self.provider.subscribe_pending_txs().await?;

        while let Some(tx_hash) = stream.next().await {
            if let Some(tx) = self.provider.get_transaction(tx_hash).await? {
                if self.is_pengu_swap(&tx).await? {
                    log_info!("$PENGU tx: {:?}", tx.hash);
                }
            }
        }
        Ok(())
    }

    async fn is_pengu_swap(&self, tx: &Transaction) -> anyhow::Result<bool> {
        Ok(tx.to == Some(self.pengu_pair_address))
    }
}
