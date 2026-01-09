## Stage 1 invariants (lock these mentally)

- Lossless
Must be able to replay the trace byte for byte later.

- Streaming only
No serde_json::Value, no full deserialization (as it can cause OOM).

- Client agnostic at the boundary
trace-rpc crate should act as a translation layer. For an new client we just need to write a new adapter and not the entire engine.

- Deterministic artifacts
Same tx, same node, same output bytes.
newtype pattern applied correctly: pay a tiny upfront cost for massive long-term flexibility and safety in a complex domain like EVM trac
## Stage 2

- create a Rust-based model of the EVM's instruction-level execution.
- parsing the raw JSON trace (e.g., the structLogs array) and converting it into your own custom data structures
- everything in the EVM is built around this 256-bit fixed-size unit: word
- using Word wrapper for U256 in evm_traces (newtype pattern applied correctly)
Chosen so that it is compatible with outputs of keccak256
This allows to change the underlying datatype and add metadata 

## Learnings
- An invariant is a property that always held across past executions. 
```
- security boundaries
- protocol assumptions
- upgrade risks
```
- Expose artifacts instead of json sturctures. As soon as you get the JSON from the node, immediately turn it into your own clean custom Rust types. Thin wrappers like serde_json::Value must not flow in code.
Bad practice => causes full tree materialization
serde_json::from_str or from_slice loads the entire JSON into memory first, then parses it.
``` serde_json::from_reader(reader)

```
- await => synchronous exec but non-blocking | the thread gets freed and returns back to tokio runtime for other opoerations
- PathBuf (owns the data on heap, can be modified) , Path (borrows reference, cannot be modified, like &str)
- stage-1 should only be about procuring traces and no parsing of the trace. The validation must be structural rather than semantic


## Conventions
- struct fields in snake_case #[(use serde(renmae=""))] while mapping json to struct using serde
- "Visitor" design pattern is fundamental to data parsing
deserializer/itr reads tokens from json obj
visitor determies what to do with those tokens


## POI's
- In Rust, when a struct implements a trait:

Inherent methods (defined directly on the struct) can be called with just the struct in scope
Trait methods can ONLY be called when the trait is imported

- In runtime taint analysis.You track:
```
- where a value originated
- how it propagates
- where it ends up

Example questions:

Does msg.sender influence storage slot X?

Does user input affect external calls?

Does an oracle value influence a transfer?
```

- Scope: &str can reference temporary data; &'static str must live forever
- macro rules
```
- $( ... ),*  : repetition pattern
- $(,)  : for trailing comma
- $halt:expr any expression (true, false, function call, const)

