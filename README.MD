Absolutely — here’s your **updated `README.md`**, keeping **your base style**, but now upgraded with your **latest Abstract build notes**, **fixed strategy details**, **300 PENGU back-run logic**, and your **donation section** at the bottom, **merged cleanly** into your original format.

---

## ✅ Final `README.md`

````markdown
# 🧊🐧 Frostbyte — Abstract Blockchain Sandwich Bot

> **Frostbyte** is an experimental, production-ready sandwich trading bot designed specifically for **Abstract Blockchain Mainnet** (`chain ID 2741`).  
> It scans the mempool for profitable opportunities to front-run and back-run transactions on the **$PENGU token**, automatically simulates profitability, and safely executes **ETH ➜ PENGU ➜ ETH** cycles — all on-chain.

---

## ✨ Key Features

✅ Runs live on **Abstract Blockchain Mainnet** — not testnet  
✅ Fully automated mempool watching & real-time profit simulation  
✅ Dynamic ETH balance management — front-run uses **50% of your live ETH balance**, reserves gas buffer  
✅ Fixed **300 PENGU** back-run size for more consistent ETH flips  
✅ One-time `approve()` for unlimited PENGU ➜ ETH swaps  
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
     ├── router.rs
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
Frostbyte 🧊🐧 Frostbyte v1.1 🧊🐧 — Start
Wallet: 0xYourWalletAddress
Heartbeat…
MempoolWatcher watching: 0xYourPenguPairAddress
Wallet balance: 0.005 ETH
Sim: Profit = ...
Profitable! Executing runs…
Front-run hash: 0x...
Back-run hash: 0x...
```

✅ **Stop anytime** with `Ctrl+C` — Frostbyte shuts down gracefully with no orphan tasks.

---

## 🔑 One-Time Setup — PENGU Approve

Before your back-run can work, you **must approve** your router once:

```
PENGU.approve("0xYOUR_ROUTER_ADDRESS", MAX_UINT256)
```

You can do this on Abstract’s block explorer, Metamask, or using `ethers.js`.
This only needs to be done once per wallet.

---

## 💡 How It Works

✅ Mempool is scanned in real-time for target swaps.
✅ Profit is simulated using your actual on-chain liquidity.
✅ **Front-run** uses **50%** of your available ETH balance — dynamic sizing.
✅ **Back-run** always trades a **fixed 300 PENGU ➜ ETH** for consistent unwinds.
✅ `tokio::select!` pattern ensures your shutdown is graceful.

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

## 💚 Donate (Optional)

If you appreciate Frostbyte, feel free to tip the dev!

**ETH:** `0x22ee052B04496d554852EA6762b3Ad98b2CF7860`

**SOL:** `5bgLMbDANsQnrs57RZtdJftkiTiyZd9gfKNseAQQHS3u`

**Sui:** `0xadc6d1dfb6152ac51cc61a4678c497f22acbf34d2a5167b24ddb688895176736`

**BTC:** `bc1qwv4ata5ygfawj2rtl6l4ggts645nee2whtcg0c`

---

## 🧊🐧 Stay frosty — and sandwich wisely!

```
