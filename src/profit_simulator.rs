use ethers::prelude::*;
use ethers::types::Address;
use crate::log_info;
use crate::abstract_swap_router::AbstractSwapRouter;

pub struct ProfitSimulator<M>
where
    M: Middleware + 'static,
{
    pub router: AbstractSwapRouter<M>,
    pub eth_address: Address,
    pub pengu_address: Address,
}

impl<M> ProfitSimulator<M>
where
    M: Middleware + 'static,
{
    pub fn new(router: AbstractSwapRouter<M>, eth: Address, pengu: Address) -> Self {
        Self { router, eth_address: eth, pengu_address: pengu }
    }

    pub async fn simulate_sandwich(&self, victim_amount: U256) -> anyhow::Result<f64> {
        let path = vec![self.eth_address, self.pengu_address];
        let out = self.router.get_amounts_out(victim_amount, path).call().await?;
        let profit = wei_to_eth(out.last().copied().unwrap_or_default());
        log_info!("Sim: Profit = {:.6} ETH", profit);
        Ok(profit)
    }
}

fn wei_to_eth(wei: U256) -> f64 {
    wei.as_u128() as f64 / 1e18
}
