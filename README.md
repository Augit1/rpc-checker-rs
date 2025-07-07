# 📦 rpc-checker-rs

[![Rust Edition](https://img.shields.io/badge/Rust-2021-orange?logo=rust)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![CI](https://github.com/Augit1/rpc-checker-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/myusername/rpc-checker-rs/actions)

A minimal CLI tool written in Rust to check the health and sync status of any Ethereum-compatible JSON-RPC endpoint (Sepolia, Gnosis, Ethereum Mainnet, etc.).

---

## 🚀 Overview

`rpc-checker-rs` sends a `eth_blockNumber` request to the specified RPC URL and returns the latest block number.  
Useful for checking node sync, testing third-party endpoints, or integrating in monitoring systems.

> Built to learn Rust, explore Ethereum infrastructure, and keep things fast and minimal.

---

## 🔧 Features

- ⚡️ Async HTTP client with [`reqwest`](https://docs.rs/reqwest/)
- 🧪 Compatible with any EVM RPC (Infura, Alchemy, Gnosis, Blast, etc.)
- 📈 Human-readable CLI output
- 🧰 Can be used in shell scripts and cron jobs
- 🛑 Clear error reporting for timeouts, malformed JSON, or down endpoints

---

## 📁 Project Structure

```
rpc-checker-rs/
├── src/
│   └── main.rs         # Rust source
├── Cargo.toml          # Dependencies & metadata
├── README.md           # You are here
```

---

## ▶️ How to Use

### 📦 Requirements

- Rust 1.70+ installed ([install Rust](https://rustup.rs))

### 🔨 Build and run

```bash
cargo run --release -- https://rpc.gnosischain.com
```

✅ Example Output

HTTP Status: 200 OK
Raw Response: {"jsonrpc":"2.0","id":1,"result":"0x27011f5"}
Latest block: 40899061

⚠️ Example of RPC failure (Sepolia public node)

HTTP Status: 522 <unknown status code>
Raw Response: error code: 522
Error: Failed to parse JSON: expected value at line 1 column 1


⸻

🛠️ Improvements You Can Make
	•	Add --json flag for machine-readable output
	•	Add retry logic for timeouts (tokio-retry)
	•	Allow checking multiple URLs from a file
	•	Display block lag from chain tip
	•	Optional colored output (e.g. green/red)

⸻

🤝 Contributing

Feel free to fork, improve, or suggest features via pull requests or issues.

⸻

📜 License

MIT © 2025 Augustin de La Brosse

⸻

💡 Tip

Want to use Sepolia or Mainnet? Use trusted RPCs like:

https://sepolia.infura.io/v3/YOUR_API_KEY
https://eth-mainnet.g.alchemy.com/v2/YOUR_API_KEY
https://rpc.gnosischain.com


⸻
