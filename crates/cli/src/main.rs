use std::path::{PathBuf};
use trace_rpc::{TraceConfig, TraceFetcher};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {

    let rpc_url = "https://eth-mainnet.g.alchemy.com/v2/eylw7FRc9kKHz41rQPRziikP-a1DiKU8".to_string();
    let out_dir = PathBuf::from("./data/raw_traces");
    let tx_hash = "0x9e63085271890a141297039b3b711913699f1ee4db1acb667ad7ce304772036b";

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