use ethers::prelude::*;
use crate::log_info;
use crate::abstract_swap_router::AbstractSwapRouter;

pub struct TxBuilder<M>
where
    M: Middleware + 'static,
{
    pub router: AbstractSwapRouter<M>,
    pub wallet_address: Address,
    pub weth_address: Address,
}

impl<M> TxBuilder<M>
where
    M: Middleware + 'static,
{
    pub fn new(router: AbstractSwapRouter<M>, wallet_address: Address, weth_address: Address) -> Self {
        Self { router, wallet_address, weth_address }
    }

    pub async fn execute_front_run(
        &self,
        eth_amount: U256,
        pengu_address: Address,
        _slippage: f64,
    ) -> anyhow::Result<TxHash> {
        log_info!("Preparing front-run: {:.6} ETH -> PENGU", wei_to_eth(eth_amount));

        let path = vec![self.weth_address, pengu_address];

        let tx = self
            .router
            .swap_exact_eth_for_tokens(
                U256::zero(),
                path,
                self.wallet_address,
                U256::from(self.get_deadline()),
            )
            .value(eth_amount);

        let pending = tx.send().await?;
        let receipt = pending.await?.unwrap();
        log_info!("Front-run hash: {:?}", receipt.transaction_hash);
        Ok(receipt.transaction_hash)
    }

    pub async fn execute_back_run(
        &self,
        pengu_amount: U256,
        pengu_address: Address,
        _slippage: f64,
    ) -> anyhow::Result<TxHash> {
        log_info!("Preparing back-run: {:.6} PENGU -> ETH", wei_to_eth(pengu_amount));

        let path = vec![pengu_address, self.weth_address];

        let tx = self
            .router
            .swap_exact_tokens_for_eth(
                pengu_amount,
                U256::zero(),
                path,
                self.wallet_address,
                U256::from(self.get_deadline()),
            );

        let pending = tx.send().await?;
        let receipt = pending.await?.unwrap();
        log_info!("Back-run hash: {:?}", receipt.transaction_hash);
        Ok(receipt.transaction_hash)
    }

    fn get_deadline(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now + (20 * 60)
    }
}

fn wei_to_eth(wei: U256) -> f64 {
    wei.as_u128() as f64 / 1e18
}
