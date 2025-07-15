# 🧊🐧 Frostbyte — Abstract Blockchain Sandwich Bot

> **Frostbyte** is an experimental, production-ready sandwich trading bot designed specifically for **Abstract Blockchain Mainnet** (chain ID `2741`).  
> It scans the mempool for profitable opportunities to front-run and back-run transactions on the **$PENGU token**, automatically simulates profitability, and safely executes **ETH ➜ PENGU ➜ ETH** cycles — all on-chain.

---

## ✨ Key Features

✅ Runs live on **Abstract Blockchain Mainnet** — not testnet  
✅ Fully automated mempool watching & real-time profit simulation  
✅ Dynamic balance management — only uses available ETH, auto-reserves gas buffer  
✅ Unlimited PENGU `approve` — back-run swaps succeed every time  
✅ Safe `Ctrl+C` shutdown — no stuck states, no unexpected errors  
✅ Fully open-source — your keys, your wallet, your profit!

---

## ⚙️ Prerequisites

Before you get started, you’ll need:

| ✅ | Requirement | Why |
| --- | --- | --- |
| 🦀 | **Rust & Cargo** | Frostbyte is written in Rust. |
| 🌐 | **Abstract Blockchain Mainnet RPC URL** | You’ll need a valid WebSocket RPC URL for Abstract Mainnet. |
| 🔑 | **Your wallet private key** | Stored securely — never share this! |

---

## 🚀 Installation

### 1️⃣ Install Rust

If you don’t have Rust & Cargo yet:  
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

Then reload your terminal:

```bash
source $HOME/.cargo/env
```

Verify it works:

```bash
rustc --version
cargo --version
```

---

### 2️⃣ Clone Frostbyte

```bash
git clone https://github.com/YOUR_USERNAME/frostbyte.git
cd frostbyte
```

---

### 3️⃣ Add your RPC URL & Private Key

In `src/main.rs` find:

```rust
let ws = Ws::connect("wss://YOUR_ABSTRACT_MAINNET_WS_RPC").await?;
let wallet: LocalWallet = "YOUR_PRIVATE_KEY_HERE"
    .parse::<LocalWallet>()?
    .with_chain_id(2741_u64); // ✅ Abstract Blockchain Mainnet
```

Replace with:

* ✅ Your **Abstract Blockchain Mainnet WebSocket RPC**
* ✅ Your wallet private key (only for this wallet)

**Always test with small funds!**

---

## 📂 Project Structure

```
Frostbyte_PublicBranch/
 ├── Cargo.toml
 ├── abis/
 │   └── AbstractSwapRouter.json
 └── src/
     ├── abstract_swap_router.rs
     ├── logger.rs
     ├── main.rs
     ├── mempool_watcher.rs
     ├── profit_simulator.rs
     ├── tx_builder.rs
```

✅ Everything you need is included — no extra ABI downloads required.

---

## 🏃‍♂️ Run Frostbyte

**Build:**

```bash
cargo build --release
```

**Run:**

```bash
cargo run --release
```

✅ If everything’s working, you’ll see:

```
Frostbyte 🧊🐧 Frostbyte v1.0 🧊🐧 — Start
Wallet: 0xYourWalletAddress
Heartbeat…
MempoolWatcher watching: 0xYourPenguPairAddress
Wallet balance: 0.005 ETH
Sim: Profit = ...
Profitable! Executing runs…
Front tx: 0x...
Back tx: 0x...
```

✅ **Stop anytime** with `Ctrl+C` — Frostbyte shuts down gracefully.

---

## 🔑 One-Time Setup — PENGU Approve

Before your back-run can work, you **must approve** your router once:

```
PENGU.approve("0xROUTER_CONTRACT_ADDRESS", MAX_UINT256)
```

You can do this on Abstract’s block explorer, Metamask, or using `ethers.js`.
This only needs to be done once per wallet.

---

## 💡 Safety Tips

✅ Use a **fresh wallet** with limited funds for testing.
✅ Never run Frostbyte with your entire balance.
✅ Always monitor your mempool traffic and profit thresholds.
✅ Understand **sandwich attacks** — they are aggressive strategies.
✅ Research Abstract Blockchain’s liquidity and router behavior.

---

## 🗒️ Current Status

🧊 Frostbyte is live, working, and stable on Abstract Blockchain Mainnet.
✨ It holds real PENGU as your “inventory” and tries to flip it back to ETH when profitable.
🛠️ More features coming soon: forced unwind mode, multi-pair scanning, and more!

---

## 👑 Credits

* 🐧 Frostbyte 🧊🐧 by \[ShamelesAbyss]
* Inspired by modern MEV bots and sandwich trading strategies.
* Built with ❤️ in Rust.

---

## 🔗 License

MIT — open-source, use at your own risk.
This bot does not guarantee profit and is intended for research and educational use only.

---

## 🌟 Show Your Support

If you find Frostbyte useful:
⭐ Star the repo
🐧 Share your PRs, feedback, and ideas!
🍕 Buy me a coffee!

---

## 🧊🐧 Stay frosty — and sandwich wisely!
