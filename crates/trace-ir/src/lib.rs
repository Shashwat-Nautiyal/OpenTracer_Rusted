mod word;
mod opcode;

pub use word::Word;
pub use opcode::{Opcode, OpcodeInfo};

use serde::{Serialize, Deserialize};
use std::sync::Arc;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction{
    pub pc: u64,
    
    #[serde(rename="op")]
    pub opcode: Opcode,

    pub gas: u64,

    #[serde(rename="gasCost")]
    pub gas_cost: Option<u64>,

    #[serde(default)]
    pub stack: Vec<Word>,

    pub depth: u64,

    #[serde(default)]
    pub memory: Option<Vec<Word>>,
}

impl Instruction {
    pub fn info(&self)->OpcodeInfo {
        self.opcode.info()
    }
}


#[cfg(test)]
mod tests {
    // brings everything from the parent module
    use super::*;
    use alloy_primitives::U256;
    use serde_json;

    #[test]
    fn test_opcode_metadata() {
        let op = Opcode::ADD;
        let info = op.info();

        assert_eq!(info.name, "ADD");
        assert_eq!(info.bytes, 0x01);
        assert_eq!(info.inputs, 2);
        assert_eq!(info.outputs, 1);
        assert!(!info.is_halt);
    }

    #[test]
    fn test_opcode_deserialization(){
        let json = r#""PUSH1""#;

        // this makes use of the cutom deserialize impl in Opcode.rs
        let op: Opcode = serde_json::from_str(json).unwrap();
        assert_eq!(op, Opcode::PUSH1);

        let invalid = r#""INVALID""#;
        let op2: Opcode = serde_json::from_str(invalid).unwrap();
        assert_eq!(op2, Opcode::INVALID);
    }

    #[test]
    fn test_instruction_parsing() {

        let json_data = r#"
        {
            "pc": 10,
            "op": "ADD",
            "gas": 50000,
            "gasCost": 3,
            "depth": 1,
            "stack": [
                "0x0000000000000000000000000000000000000000000000000000000000000001",
                "0x0000000000000000000000000000000000000000000000000000000000000002"
            ]
        }
        "#;

        // this uses derived deserialize impl
        let instruction: Instruction = serde_json::from_str(json_data).expect("Failed to parse instruction");

        assert_eq!(instruction.pc, 10);
        assert_eq!(instruction.opcode, Opcode::ADD);
        assert_eq!(instruction.gas, 50000);
        assert_eq!(instruction.depth, 1);
        
        // Check Stack
        assert_eq!(instruction.stack.len(), 2);
        assert_eq!(instruction.stack[0], Word(U256::from(1)));
        assert_eq!(instruction.stack[1], Word(U256::from(2)));

        
    }


    #[test]
    fn test_optional_fields() {
        let json_data = r#"
        {
            "pc": 0,
            "op": "STOP",
            "gas": 0,
            "depth": 0,
            "stack": []
        }
        "#;

        let instruction: Instruction = serde_json::from_str(json_data).expect("Should parse minimal instruction");
        
        // Should be None
        assert_eq!(instruction.gas_cost, None); 
        assert_eq!(instruction.memory, None);   
    }


    
}