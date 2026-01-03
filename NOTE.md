##Stage 1 invariants (lock these mentally)

- Lossless
Must be able to replay the trace byte for byte later.

- Streaming only
No serde_json::Value, no full deserialization (as it can cause OOM).

- Client agnostic at the boundary
trace-rpc crate should act as a translation layer. For an new client we just need to write a new adapter and not the entire engine.

- Deterministic artifacts
Same tx, same node, same output bytes.

## Learnings
- Expose artifacts instead of json sturctures. As soon as you get the JSON from the node, immediately turn it into your own clean custom Rust types.
- await => synchronous exec but non-blocking | the thread gets freed and returns back to tokio runtime for other opoerations
- PathBuf (owns the data on heap, can be modified) , Path (borrows reference, cannot be modified, like &str)


## Conventions
- struct fields in snake_case #[(use serde(renmae=""))] while mapping json to struct using serde