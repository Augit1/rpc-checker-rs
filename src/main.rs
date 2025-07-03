
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = std::env::args().nth(1).expect("Usage: rpc-checker <RPC_URL>");
    let client = Client::new();
  
    let payload = json!({
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "params": [],
        "id": 1
    });

    let resp = client.post(&rpc_url)
        .json(&payload)
        .send()
        .await?;

    let status = resp.status();
    let text = resp.text().await?;

    println!("HTTP Status: {}", status);
    println!("Raw Response: {}", text);

    let json: serde_json::Value = match serde_json::from_str(&text) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("\x1b[31mError: Failed to parse JSON:\x1b[0m {}", e);
            return Ok(());
    }
};

    if let Some(result) = json.get("result") {
        let block_hex = result.as_str().unwrap_or("0x0");
        let block_num = u64::from_str_radix(&block_hex.trim_start_matches("0x"), 16).unwrap_or(0);
        println!("\x1b[32mLatest block: {}\x1b[0m", block_num);
    } else {
        eprintln!("\x1b[31mInvalid response from node:\x1b[0m {:?}", json);
    }

    Ok(())
}
