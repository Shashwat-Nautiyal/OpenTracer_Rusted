use std::path::{Path, PathBuf};
use trace_rpc::{TraceConfig, TraceFetcher};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {

    let rpc_url = "https://ethereum-sepolia.blockpi.network/v1/rpc/public".to_string();
    let out_dir = PathBuf::from("./data/raw_traces");
    let tx_hash = "";

    let config = TraceConfig{
        rpc_url, 
        out_dir
    };
    
    let fetcher = TraceFetcher::new(config);

    match fetcher.fetch_transaction(tx_hash).await {
        Ok(raw_trace) => {
            println!("success!");
            println!("trace saved to: {:?} ", raw_trace.trace_path);
        },
        Err(e) => {
            eprintln!("Error fetching tx trace: {:?}", e);
        }
    }

    Ok(())


}