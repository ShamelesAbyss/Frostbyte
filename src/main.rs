use ethers::prelude::*;
use ethers::providers::{Provider, Ws};
use ethers::middleware::SignerMiddleware;
use ethers::signers::{LocalWallet, Signer};
use std::sync::Arc;
use std::time::Duration;
use tokio::{time::sleep, signal};
use std::str::FromStr;
use colored::Colorize;

mod logger;
mod abstract_swap_router;
mod mempool_watcher;
mod profit_simulator;
mod tx_builder;

use abstract_swap_router::AbstractSwapRouter;
use mempool_watcher::MempoolWatcher;
use profit_simulator::ProfitSimulator;
use tx_builder::TxBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log_info!("Frostbyte v1.1 🧊🐧 — Start");

    // TODO: Replace with your own Abstract Blockchain Mainnet WS URL
    let ws = Ws::connect("wss://abstract-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_KEY").await?;
    let provider = Arc::new(Provider::new(ws));

    // TODO: Replace with your own wallet private key
    let wallet: LocalWallet = "YOUR_PRIVATE_KEY"
        .parse::<LocalWallet>()?
        .with_chain_id(2741_u64); // Abstract Blockchain Mainnet chain ID

    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));
    log_info!("Wallet: {:?}", wallet.address());

    // TODO: Replace with your contract addresses
    let eth = Address::from_str("0xYOUR_ETH_ADDRESS")?;
    let pengu = Address::from_str("0xYOUR_PENGU_TOKEN_ADDRESS")?;
    let router_addr = Address::from_str("0xYOUR_ROUTER_ADDRESS")?;

    let router = AbstractSwapRouter::new(router_addr, client.clone());

    let mempool = MempoolWatcher::new(provider.clone(), pengu);
    let profit_sim = ProfitSimulator::new(router.clone(), eth, pengu);
    let tx_builder = TxBuilder::new(router.clone(), wallet.address(), eth);

    tokio::spawn(async move {
        if let Err(e) = mempool.watch_pending().await {
            log_error!("Mempool error: {:?}", e);
        }
    });

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                log_info!("Shutting down Frostbyte gracefully (Ctrl+C).");
                break;
            }
            _ = async {
                log_info!("Heartbeat…");

                let eth_balance = provider.get_balance(wallet.address(), None).await?;
                let eth_balance_f64 = wei_to_eth(eth_balance);
                log_info!("Wallet balance: {:.6} ETH", eth_balance_f64);

                if eth_balance_f64 < 0.001 {
                    log_info!("Balance too low, skipping this round.");
                } else {
                    let victim_amount = eth_balance / 2;
                    let profit = profit_sim.simulate_sandwich(victim_amount).await?;

                    if profit > 0.005 {
                        log_info!("Profitable! Executing runs…");

                        match tx_builder.execute_front_run(victim_amount, pengu, 0.01).await {
                            Ok(tx) => log_info!("Front tx: {:?}", tx),
                            Err(e) => log_error!("Front-run failed: {:?}", e),
                        }

                        // Fixed 300 PENGU for back-run
                        let back_run_amount = U256::from(300u64) * U256::exp10(18);

                        match tx_builder.execute_back_run(back_run_amount, pengu, 0.01).await {
                            Ok(tx) => log_info!("Back tx: {:?}", tx),
                            Err(e) => log_error!("Back-run failed: {:?}", e),
                        }
                    } else {
                        log_info!("No profit found.");
                    }
                }

                sleep(Duration::from_secs(15)).await;

                Ok::<(), anyhow::Error>(())
            } => {}
        }
    }

    Ok(())
}

fn wei_to_eth(wei: U256) -> f64 {
    wei.as_u128() as f64 / 1e18
}
