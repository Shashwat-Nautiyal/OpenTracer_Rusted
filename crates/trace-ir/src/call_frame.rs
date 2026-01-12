use serde::{Serialize, Deserialize};
use crate::{Word, Opcode, Instruction};
use alloy_primitives::Address;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallType{
    Call,
    StaticCall,
    DelegateCall,
    Create,
    Create2,
    CallCode,
    Root,           // top-level trnx
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CallFrame {
    pub call_type: CallType,
    pub to: Address,            // 'to' is usually the address of the code currently executing.
    pub from: Address,

    pub value: Word,

    pub return_data: Vec<u8>,
    pub calldata: Vec<u8>,

    pub gas_limit: u64,
    pub gas_used: u64,

    //result of this frame
    pub success: bool,
    pub error: Option<String>,

    pub instructions: Vec<Instruction>,
    pub children: Vec<CallFrame>


}

impl CallFrame {
    pub fn new(call_type: CallType, from: Address, to: Address, gas: u64)-> Self {
        Self {
            call_type,
            from,
            to,
            value: Word::ZERO,
            return_data: Vec::new(),
            calldata: Vec::new(),
            gas_limit: gas,
            gas_used: 0,
            success: true,
            error: None,
            instructions: Vec::new(),
            children: Vec::new(),
        }
    }
}

