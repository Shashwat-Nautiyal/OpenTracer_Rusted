use std::path::PathBuf;

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
    out_dir: PathBuf,
}

pub fn debug_trace_payload(tx_hash: String) -> String {
    // raw string
    format!{
        r#"{{
            "jsonrpc": "2.0",
            "id": "1",
            "method": "debug_traceTransaction",
            "params": [
                "{}",
                {{
                    "disableStack": false,
                    "disableMemory": true,
                    "disableStorage": true,
                }}
            ]
        }}"#,
        tx_hash
    }
}

pub fn receipt_payload(tx_hash: String) -> String{
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

#[cfg(test)]
mod tests {
    use super::*;

    
   
}
