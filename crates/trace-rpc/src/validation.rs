use core::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use anyhow::{Result, Context};
use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};
use serde_json;



// checks for truncated or corrupted JSON
fn validate_json_well_formed(path: &Path) -> Result<()> {
    let file = File::open(path)
        .context("could not open trace file for validation")?;
    let reader = BufReader::new(file);

    // parses the JSON lazily, without loading the entire file into memory
    // by creating a streaming JSON deserializer
    let mut de = serde_json::Deserializer::from_reader(reader);

    // special type that says: "I don't care what the value is, just consume it and throw it away
    let _ : serde::de::IgnoredAny = Deserialize::deserialize(&mut de)
                                    .context("Invalid or truncated json")?;

    // checks for extra data after the json object
    de.end().context("trailing data after JSON")?;


    Ok(())
}

fn validate_rpc_envelope(path: &Path)->Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut de = serde_json::Deserializer::from_reader(reader);
    de.deserialize_any(RpcEnvelopeVisitor)
        .map_err(|e| anyhow::anyhow!("Invalid Rpc envelope: {}", e))?;

    Ok(())
}


struct RpcEnvelopeVisitor ;

impl<'de> Visitor<'de> for RpcEnvelopeVisitor {
    type Value= ();

    fn expecting(&self, formatter: &mut fmt::Formatter)-> fmt::Result {
        formatter.write_str("A valid JSON-RPC 2.0 response object")
    }

    fn visit_map<A> (self, mut map: A)-> Result<Self::Value, A::Error>
    where 
        A: MapAccess<'de>
    {
        let mut has_result = false;
        let mut has_error = false;
        let mut has_jsonrpc = false;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "jsonrpc" => {
                    let version: String = map.next_value()?;
                    if version == "2.0" {
                        has_jsonrpc = true;
                    }
                },
                "id" => {
                    let _ = map.next_value::<de::IgnoredAny>()?;
                },
                "result" => {
                    has_result = true;
                    map.next_value::<de::IgnoredAny>()?;
                },
                "error" => {
                    has_error = true;
                    map.next_value::<de::IgnoredAny>()?;

                },
                _ =>{
                    map.next_value::<de::IgnoredAny>()?;

                }

            }
        }

        if !has_jsonrpc {
            return Err(de::Error::custom("Missing/Invalid 'jsonrpc' field"));
        }
        if has_error {
            return Err(de::Error::custom("Rpc returned an error"));
        }
        if !has_result {
            return Err(de::Error::custom("Rpc response missing 'result' field"));
        }

        Ok(())
    }

}

pub fn validate_trace_file(path: &Path)->Result<()>{

    validate_rpc_envelope(path).context("Trace file has invalid rpc response")?;
    validate_json_well_formed(path).context("Trace file is not well-formed JSON (correupted/truncated)")?;

    Ok(())
}
