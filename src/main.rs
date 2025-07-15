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
    log_info!("Frostbyte v1.0 🧊🐧 — Start");

    // ✅ PLACEHOLDER: Replace with your Abstract Blockchain Mainnet WS URL
    let ws = Ws::connect("wss://YOUR_ABSTRACT_MAINNET_WS_RPC").await?;
    let provider = Arc::new(Provider::new(ws));

    // ✅ PLACEHOLDER: Replace with your wallet private key
    let wallet: LocalWallet = "YOUR_PRIVATE_KEY_HERE"
        .parse::<LocalWallet>()?
        .with_chain_id(2741_u64); // ✅ Abstract Blockchain Mainnet

    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));
    log_info!("Wallet: {:?}", wallet.address());

    // ✅ PLACEHOLDER addresses
    let eth = Address::from_str("0xWETH_ADDRESS")?;
    let pengu = Address::from_str("0xPENGU_TOKEN_ADDRESS")?;
    let router_addr = Address::from_str("0xROUTER_CONTRACT_ADDRESS")?;

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

                let balance = provider.get_balance(wallet.address(), None).await?;
                let eth_balance = wei_to_eth(balance);
                log_info!("Wallet balance: {:.6} ETH", eth_balance);

                let gas_reserve = U256::from(5e14 as u64); // ~0.0005 ETH reserved for gas

                if balance > gas_reserve {
                    let victim_amount = (balance - gas_reserve) / 2;

                    let profit = profit_sim.simulate_sandwich(victim_amount).await.unwrap_or(0.0);

                    if profit > 0.005 {
                        log_info!("Profitable! Executing runs…");

                        let front = tx_builder.execute_front_run(victim_amount, pengu, 0.01).await;
                        match front {
                            Ok(tx) => log_info!("Front tx: {:?}", tx),
                            Err(e) => log_error!("Front-run failed: {:?}", e),
                        }

                        let back = tx_builder.execute_back_run(victim_amount, pengu, 0.01).await;
                        match back {
                            Ok(tx) => log_info!("Back tx: {:?}", tx),
                            Err(e) => log_error!("Back-run failed: {:?}", e),
                        }
                    } else {
                        log_info!("No profit found.");
                    }
                } else {
                    log_info!("Balance too low for safe gas reserve, skipping round.");
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
