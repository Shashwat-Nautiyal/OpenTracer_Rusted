use crate::{Opcode, Word, Instruction, CallFrame, CallType};
use alloy_primitives::Address;
use anyhow::{Result, anyhow};

pub struct TraceAnalyzer;

impl TraceAnalyzer {

    pub fn build_call_tree(instructions: Vec<Instruction>) -> Result<CallFrame>{
        if instructions.is_empty() {
            return Err(anyhow!("Trace is empty!"))
        }

        let root_frame = CallFrame::new(
            CallType::Root,
            Address::ZERO,
            Address::ZERO,
            instructions[0].gas
        );

        let mut frame_stack: Vec<CallFrame> = Vec::new();
        frame_stack.push(root_frame);

        let mut previous_depth = instructions[0].depth;

        for instr in instructions {
            let current_depth = instr.depth;

            if current_depth > previous_depth {
                let parent = frame_stack.last_mut().expect("Stack Underflow!!");
                let triggering_op = parent.instructions.last().map(|i| i.opcode).unwrap_or(Opcode::INVALID);

                let calltype: CallType = match triggering_op {
                    Opcode::CALL => CallType::Call,
                    Opcode::CALLCODE => CallType::CallCode,
                    Opcode::DELEGATECALL => CallType::DelegateCall,
                    Opcode::STATICCALL => CallType::StaticCall,
                    Opcode::CREATE => CallType::Create,
                    Opcode::CREATE2 => CallType::Create2,
                    _ => CallType::Call,
                };

                let new_frame = CallFrame::new(
                    calltype,
                    Address::ZERO,
                    Address::ZERO,
                    instr.gas
                );

                frame_stack.push(new_frame);

            }else if previous_depth < current_depth {

                // for geth client depth starts from 1, for erigon starts from 0 
                while frame_stack.len() as u64 > current_depth {
                    if let Some(finished_frame) = frame_stack.pop(){
                        if let Some(parent) = frame_stack.last_mut(){
                            parent.children.push(finished_frame);
                        }
                    }
                }
            }

            if let Some(current_frame) = frame_stack.last_mut() {
                if instr.opcode == Opcode::REVERT {
                    current_frame.success = false;
                    current_frame.error = Some("Reverted".to_string());
                }

                current_frame.instructions.push(instr.clone());
            }

            previous_depth = current_depth;
        }

        while frame_stack.len() > 1 {
            if let Some(finished_frame) = frame_stack.pop(){
                if let Some(parent) = frame_stack.last_mut(){
                    parent.children.push(finished_frame);
                }
            }
        }

        frame_stack.pop().ok_or_else(|| anyhow!("Stack corrupted during reconstruction!!"))
    }
}