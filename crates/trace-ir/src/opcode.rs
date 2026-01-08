use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy)]
pub struct opcodeInfo {
    pub name : & 'static str,
    pub bytes : u8,
    pub inputs:  u8,
    pub outputs : u8,
    pub is_call : bool,
    pub is_halt: bool,

}


macro_rules! define_opcodes {
    (
        $($name:ident = $byte.literal {in: $in:literal, out: $out:literal, halt: $halt.expr, call: $call.expr}),*
        &(,)?
    ) = > {

        #[derive(Debug, Clone, Copy. Eq, PartialEq, Hash)]
        #[repr(u8)]
        pub enum Opcode {
            $($name = $byte, )*
            INVALID: 0xFE
        }

        impl Opcode {
            
        }
    }
}