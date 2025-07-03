use reqwest::Client;
use serde::{Deserialize, Serialize};
use clap::Parser;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    rpc_url: String,

    #[arg(short, long)]
    json: bool,
}

#[derive(Deserialize, Debug)]
struct RpcResponse {
    result: String,
}

#[derive(Serialize)]
struct Output {
    block: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "params": [],
        "id": 1
    });

    let resp = match client.post(&args.rpc_url).json(&payload).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("\x1b[31mRequest failed:\x1b[0m {}", e);
            return Ok(());
        }
    };

    let status = resp.status();
    let text = resp.text().await?;

    println!("HTTP Status: {}", status);
    println!("Raw Response: {}", text);

    let parsed: RpcResponse = match serde_json::from_str(&text) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("\x1b[31mError: Failed to parse JSON:\x1b[0m {}", e);
            return Ok(());
        }
    };

    let block_hex = parsed.result.trim_start_matches("0x");
    let block_num = u64::from_str_radix(block_hex, 16).unwrap_or(0);

    if args.json {
        let output = Output { block: block_num };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("\x1b[32mLatest block: {}\x1b[0m", block_num);
    }

    Ok(())
}
