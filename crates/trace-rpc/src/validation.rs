use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use anyhow::{Context, Result, bail};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct RpcResponse {
    result : Option<TraceResult>,
    error : Option<RpcError>,    
}

#[derive(Deserialize, Debug)]
struct RpcError {
    code: i64,
    msg : String,
}

#[derive(Deserialize, Debug)]
struct TraceResult {
    // tells serde to map the type's field to to a diff name in json
    #[serde(rename="structLogs")]
    struct_logs: Vec<MinimalLog>,
}

#[derive(Deserialize, Debug)]
struct MinimalLog {
    pc: u64,
    op: String,
    gas: u64,
    depth: u64,
}


pub fn validate_trace_file(path: &Path) -> Result<()> {

    let file = File::open(path).context("could not open trace file for validaiton")?;
    let reader = BufReader::new(file);

    let res: RpcResponse = serde_json::from_reader(reader)
                .context("Failed to parse json structure. File might be incomplete")?;
    
    if let Some(err) = res.error {
        bail!("RPC error returned: [{}] {}", err.code, err.msg);
    }

    let result = res.result.context("Response contains neither 'result' nor 'error'. Malformed RPC response ")?;

    if result.struct_logs.is_empty() {
        bail!("Validation failed! 'structLogs' is empty.");
    }

    Ok(())
}