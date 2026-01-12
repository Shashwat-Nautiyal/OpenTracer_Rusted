use std::path::{PathBuf};
use trace_rpc::{TraceConfig, TraceFetcher};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {

    let rpc_url = "https://mainnet.gateway.tenderly.co/68FIYvi1epfk2HlzP0XAMz".to_string();
    let out_dir = PathBuf::from("./data/raw_traces");
    let tx_hash = "0x2d8edc881796aff96a5c6177665c7b3c7266108f23c9732a8c21a9771277d8c5";

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