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
