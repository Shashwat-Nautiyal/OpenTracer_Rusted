use std::path::{Path,PathBuf};
use anyhow::{Result, Context};
use serde_json::json;
use reqwest::Client;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

// one fully acquired tx trace
pub struct RawTrace {
    pub tx_hash: String,
    pub trace_path: PathBuf,  // saved raw exec trace from debug_traceTransaction
    pub receipt_path: PathBuf,  // from eth_getTransactionReceipt
    pub metadata_path: PathBuf,
}

// configuration for acquiring traces
pub struct TraceConfig {
    pub rpc_url: String,
    pub out_dir: PathBuf,
}

// will return string to make rpc req
pub fn debug_trace_payload(tx_hash: &str) -> String {
    // raw string
    format!(
        r#"{{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "debug_traceTransaction",
            "params": [
                "{}",
                {{
                    "disableStack": false,
                    "disableMemory": true,
                    "disableStorage": true
                }}
            ]
        }}"#,
        tx_hash
    )
}

pub fn receipt_payload(tx_hash: &str) -> String{
    format!(
        r#"
            {{
                "jsonrpc": "2.0",
                "id": "1",
                "method": "eth_getTransactionReceipt",
                "params": ["{}"]

            }}
        "#,
        tx_hash
    )
}

pub struct TraceFetcher {
    client: Client,
    config: TraceConfig,
}

impl TraceFetcher {
    pub fn new(config: TraceConfig) -> Self {
        Self{
            client: Client::new(), 
            config
        }
    }

    pub async fn fetch_transaction(&self, tx_hash: &str) -> Result<RawTrace> {
        
        let base_path = self.config.out_dir.join(tx_hash);

        if !base_path.exists() {
            fs::create_dir_all(&base_path).await.context("Failed to create tx directory")?;
        }

        let trace_path = base_path.join("trace.json");
        let receipt_path = base_path.join("receipt.json");
        let metadata_path = base_path.join("metadata.json");

        // println!("[{}] Requesting Debug trace ...", tx_hash);
        // let trace_rpc_payload = debug_trace_payload(tx_hash);
        // self.stream_rpc_response(&trace_rpc_payload, &trace_path).await
        // .context("Failed to download trace")?;

        println!("[{}] Requesting receipt ...", tx_hash);
        let receipt_rpc_payload = receipt_payload(tx_hash);
        self.stream_rpc_response(&receipt_rpc_payload, &receipt_path).await.
        context("Failed to download receipt")?;

        let metadata = json!({
            "tx_hash": tx_hash,
            "fetched_at": chrono::Utc::now().to_rfc3339(),
            "rpc_url" : self.config.rpc_url,
            "version": "1.0"
        });

        fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?).await?;

        Ok(RawTrace{
            tx_hash: tx_hash.to_string(),
            trace_path,
            receipt_path,
            metadata_path,
        })


    }

    // Path is borrowed and cannot be modified
    async fn stream_rpc_response(&self, payload: &str, out_path: &Path ) -> Result<()>{
        let mut res = self.client
        .post(&self.config.rpc_url)
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await?;

        res.error_for_status_ref()?;

        let mut file = File::create(out_path).await?;

        while let Some(chunk) = res.chunk().await? {
            file.write_all(&chunk).await?;
        }

        file.flush().await?;
        Ok(())
    }


}


