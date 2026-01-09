use serde::{ Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct OpcodeInfo {
    pub name : & 'static str,
    pub bytes : u8,
    pub inputs:  u8,
    pub outputs : u8,
    pub is_call : bool,
    pub is_halt: bool,

}


macro_rules! define_opcodes {
    (
        $($name:ident = $byte:literal {in: $in:literal, out: $out:literal, halt: $halt:expr, call: $call:expr}),*
        $(,)?
    ) => {

        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize)]
        #[repr(u8)]    // means the enum uses the actual byte as its discriminator.
        pub enum Opcode {
            $($name = $byte, )*
            INVALID = 0xFE
        }

        impl Opcode {

            pub fn from_u8(hx: u8)-> Self {

                match hx {
                    $( $byte => Opcode::$name, )*
                    _ => Opcode::INVALID,
                }

            }

            pub fn info(&self) -> OpcodeInfo {
                match self {
                    $(
                        Opcode::$name => OpcodeInfo{
                            name: stringify!($name),
                            bytes: $byte,
                            inputs: $in,
                            outputs: $out,
                            is_call: $call,
                            is_halt: $halt,
                        },
                    )*
                    
                    Opcode::INVALID => OpcodeInfo{
                        name: "INVALID",
                        bytes: 0xFE,
                        inputs: 0,
                        outputs: 0,
                        is_call: false,
                        is_halt: true,
                    }
                    
                }
            }
        }
        
        impl <'de> Deserialize <'de> for Opcode {
            fn deserialize<D>(deserializer : D) -> Result<Self, D::Error> 
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                match s.as_str() {
                    $( stringify!($name) => Ok(Opcode::$name), )*
                    _ => Ok(Opcode::INVALID),
                    
                }
            }
        }
    }

}

define_opcodes! {
    // 0x00 - Stop & Arithmetic
    STOP       = 0x00 { in: 0, out: 0, halt: true,  call: false },
    ADD        = 0x01 { in: 2, out: 1, halt: false, call: false },
    MUL        = 0x02 { in: 2, out: 1, halt: false, call: false },
    SUB        = 0x03 { in: 2, out: 1, halt: false, call: false },
    DIV        = 0x04 { in: 2, out: 1, halt: false, call: false },
    SDIV       = 0x05 { in: 2, out: 1, halt: false, call: false },
    MOD        = 0x06 { in: 2, out: 1, halt: false, call: false },
    SMOD       = 0x07 { in: 2, out: 1, halt: false, call: false },
    ADDMOD     = 0x08 { in: 3, out: 1, halt: false, call: false },
    MULMOD     = 0x09 { in: 3, out: 1, halt: false, call: false },
    EXP        = 0x0A { in: 2, out: 1, halt: false, call: false },
    SIGNEXTEND = 0x0B { in: 2, out: 1, halt: false, call: false },

    // 0x10 - Comparison & Bitwise
    LT     = 0x10 { in: 2, out: 1, halt: false, call: false },
    GT     = 0x11 { in: 2, out: 1, halt: false, call: false },
    SLT    = 0x12 { in: 2, out: 1, halt: false, call: false },
    SGT    = 0x13 { in: 2, out: 1, halt: false, call: false },
    EQ     = 0x14 { in: 2, out: 1, halt: false, call: false },
    ISZERO = 0x15 { in: 1, out: 1, halt: false, call: false },
    AND    = 0x16 { in: 2, out: 1, halt: false, call: false },
    OR     = 0x17 { in: 2, out: 1, halt: false, call: false },
    XOR    = 0x18 { in: 2, out: 1, halt: false, call: false },
    NOT    = 0x19 { in: 1, out: 1, halt: false, call: false },
    BYTE   = 0x1A { in: 2, out: 1, halt: false, call: false },
    SHL    = 0x1B { in: 2, out: 1, halt: false, call: false },
    SHR    = 0x1C { in: 2, out: 1, halt: false, call: false },
    SAR    = 0x1D { in: 2, out: 1, halt: false, call: false },

    // 0x20 - SHA3
    SHA3 = 0x20 { in: 2, out: 1, halt: false, call: false },

    // 0x30 - Environmental
    ADDRESS      = 0x30 { in: 0, out: 1, halt: false, call: false },
    BALANCE      = 0x31 { in: 1, out: 1, halt: false, call: false },
    ORIGIN       = 0x32 { in: 0, out: 1, halt: false, call: false },
    CALLER       = 0x33 { in: 0, out: 1, halt: false, call: false },
    CALLVALUE    = 0x34 { in: 0, out: 1, halt: false, call: false },
    CALLDATALOAD = 0x35 { in: 1, out: 1, halt: false, call: false },
    CALLDATASIZE = 0x36 { in: 0, out: 1, halt: false, call: false },
    CALLDATACOPY = 0x37 { in: 3, out: 0, halt: false, call: false },
    CODESIZE     = 0x38 { in: 0, out: 1, halt: false, call: false },
    CODECOPY     = 0x39 { in: 3, out: 0, halt: false, call: false },
    GASPRICE     = 0x3A { in: 0, out: 1, halt: false, call: false },
    EXTCODESIZE  = 0x3B { in: 1, out: 1, halt: false, call: false },
    EXTCODECOPY  = 0x3C { in: 4, out: 0, halt: false, call: false },
    RETURNDATASIZE = 0x3D { in: 0, out: 1, halt: false, call: false },
    RETURNDATACOPY = 0x3E { in: 3, out: 0, halt: false, call: false },
    EXTCODEHASH  = 0x3F { in: 1, out: 1, halt: false, call: false },

    // 0x40 - Block
    BLOCKHASH   = 0x40 { in: 1, out: 1, halt: false, call: false },
    COINBASE    = 0x41 { in: 0, out: 1, halt: false, call: false },
    TIMESTAMP   = 0x42 { in: 0, out: 1, halt: false, call: false },
    NUMBER      = 0x43 { in: 0, out: 1, halt: false, call: false },
    DIFFICULTY  = 0x44 { in: 0, out: 1, halt: false, call: false }, // PREVRANDAO
    GASLIMIT    = 0x45 { in: 0, out: 1, halt: false, call: false },
    CHAINID     = 0x46 { in: 0, out: 1, halt: false, call: false },
    SELFBALANCE = 0x47 { in: 0, out: 1, halt: false, call: false },
    BASEFEE     = 0x48 { in: 0, out: 1, halt: false, call: false },
    BLOBHASH    = 0x49 { in: 1, out: 1, halt: false, call: false },

    // 0x50 - Stack & Memory
    POP      = 0x50 { in: 1, out: 0, halt: false, call: false },
    MLOAD    = 0x51 { in: 1, out: 1, halt: false, call: false },
    MSTORE   = 0x52 { in: 2, out: 0, halt: false, call: false },
    MSTORE8  = 0x53 { in: 2, out: 0, halt: false, call: false },
    SLOAD    = 0x54 { in: 1, out: 1, halt: false, call: false },
    SSTORE   = 0x55 { in: 2, out: 0, halt: false, call: false },
    JUMP     = 0x56 { in: 1, out: 0, halt: false, call: false },
    JUMPI    = 0x57 { in: 2, out: 0, halt: false, call: false },
    PC       = 0x58 { in: 0, out: 1, halt: false, call: false },
    MSIZE    = 0x59 { in: 0, out: 1, halt: false, call: false },
    GAS      = 0x5A { in: 0, out: 1, halt: false, call: false },
    JUMPDEST = 0x5B { in: 0, out: 0, halt: false, call: false },
    TLOAD    = 0x5C { in: 1, out: 1, halt: false, call: false }, // EIP-1153
    TSTORE   = 0x5D { in: 2, out: 0, halt: false, call: false }, // EIP-1153

    // 0x60 - 0x7F: PUSH Operations
    PUSH0  = 0x5F { in: 0, out: 1, halt: false, call: false },
    PUSH1  = 0x60 { in: 0, out: 1, halt: false, call: false },
    PUSH2  = 0x61 { in: 0, out: 1, halt: false, call: false },
    PUSH3  = 0x62 { in: 0, out: 1, halt: false, call: false },
    PUSH4  = 0x63 { in: 0, out: 1, halt: false, call: false },
    PUSH5  = 0x64 { in: 0, out: 1, halt: false, call: false },
    PUSH6  = 0x65 { in: 0, out: 1, halt: false, call: false },
    PUSH7  = 0x66 { in: 0, out: 1, halt: false, call: false },
    PUSH8  = 0x67 { in: 0, out: 1, halt: false, call: false },
    PUSH9  = 0x68 { in: 0, out: 1, halt: false, call: false },
    PUSH10 = 0x69 { in: 0, out: 1, halt: false, call: false },
    PUSH11 = 0x6A { in: 0, out: 1, halt: false, call: false },
    PUSH12 = 0x6B { in: 0, out: 1, halt: false, call: false },
    PUSH13 = 0x6C { in: 0, out: 1, halt: false, call: false },
    PUSH14 = 0x6D { in: 0, out: 1, halt: false, call: false },
    PUSH15 = 0x6E { in: 0, out: 1, halt: false, call: false },
    PUSH16 = 0x6F { in: 0, out: 1, halt: false, call: false },
    PUSH17 = 0x70 { in: 0, out: 1, halt: false, call: false },
    PUSH18 = 0x71 { in: 0, out: 1, halt: false, call: false },
    PUSH19 = 0x72 { in: 0, out: 1, halt: false, call: false },
    PUSH20 = 0x73 { in: 0, out: 1, halt: false, call: false },
    PUSH21 = 0x74 { in: 0, out: 1, halt: false, call: false },
    PUSH22 = 0x75 { in: 0, out: 1, halt: false, call: false },
    PUSH23 = 0x76 { in: 0, out: 1, halt: false, call: false },
    PUSH24 = 0x77 { in: 0, out: 1, halt: false, call: false },
    PUSH25 = 0x78 { in: 0, out: 1, halt: false, call: false },
    PUSH26 = 0x79 { in: 0, out: 1, halt: false, call: false },
    PUSH27 = 0x7A { in: 0, out: 1, halt: false, call: false },
    PUSH28 = 0x7B { in: 0, out: 1, halt: false, call: false },
    PUSH29 = 0x7C { in: 0, out: 1, halt: false, call: false },
    PUSH30 = 0x7D { in: 0, out: 1, halt: false, call: false },
    PUSH31 = 0x7E { in: 0, out: 1, halt: false, call: false },
    PUSH32 = 0x7F { in: 0, out: 1, halt: false, call: false },

    // 0x80 - 0x9F: DUP & SWAP
    DUP1   = 0x80 { in: 1, out: 2, halt: false, call: false },
    DUP2   = 0x81 { in: 2, out: 3, halt: false, call: false },
    DUP3   = 0x82 { in: 3, out: 4, halt: false, call: false },
    DUP4   = 0x83 { in: 4, out: 5, halt: false, call: false },
    DUP5   = 0x84 { in: 5, out: 6, halt: false, call: false },
    DUP6   = 0x85 { in: 6, out: 7, halt: false, call: false },
    DUP7   = 0x86 { in: 7, out: 8, halt: false, call: false },
    DUP8   = 0x87 { in: 8, out: 9, halt: false, call: false },
    DUP9   = 0x88 { in: 9, out: 10, halt: false, call: false },
    DUP10  = 0x89 { in: 10, out: 11, halt: false, call: false },
    DUP11  = 0x8A { in: 11, out: 12, halt: false, call: false },
    DUP12  = 0x8B { in: 12, out: 13, halt: false, call: false },
    DUP13  = 0x8C { in: 13, out: 14, halt: false, call: false },
    DUP14  = 0x8D { in: 14, out: 15, halt: false, call: false },
    DUP15  = 0x8E { in: 15, out: 16, halt: false, call: false },
    DUP16  = 0x8F { in: 16, out: 17, halt: false, call: false },

    SWAP1  = 0x90 { in: 2, out: 2, halt: false, call: false },
    SWAP2  = 0x91 { in: 3, out: 3, halt: false, call: false },
    SWAP3  = 0x92 { in: 4, out: 4, halt: false, call: false },
    SWAP4  = 0x93 { in: 5, out: 5, halt: false, call: false },
    SWAP5  = 0x94 { in: 6, out: 6, halt: false, call: false },
    SWAP6  = 0x95 { in: 7, out: 7, halt: false, call: false },
    SWAP7  = 0x96 { in: 8, out: 8, halt: false, call: false },
    SWAP8  = 0x97 { in: 9, out: 9, halt: false, call: false },
    SWAP9  = 0x98 { in: 10, out: 10, halt: false, call: false },
    SWAP10 = 0x99 { in: 11, out: 11, halt: false, call: false },
    SWAP11 = 0x9A { in: 12, out: 12, halt: false, call: false },
    SWAP12 = 0x9B { in: 13, out: 13, halt: false, call: false },
    SWAP13 = 0x9C { in: 14, out: 14, halt: false, call: false },
    SWAP14 = 0x9D { in: 15, out: 15, halt: false, call: false },
    SWAP15 = 0x9E { in: 16, out: 16, halt: false, call: false },
    SWAP16 = 0x9F { in: 17, out: 17, halt: false, call: false },

    // 0xA0 - Logging
    LOG0   = 0xA0 { in: 2, out: 0, halt: false, call: false },
    LOG1   = 0xA1 { in: 3, out: 0, halt: false, call: false },
    LOG2   = 0xA2 { in: 4, out: 0, halt: false, call: false },
    LOG3   = 0xA3 { in: 5, out: 0, halt: false, call: false },
    LOG4   = 0xA4 { in: 6, out: 0, halt: false, call: false },

    // 0xF0 - System
    CREATE       = 0xF0 { in: 3, out: 1, halt: false, call: true  },
    CALL         = 0xF1 { in: 7, out: 1, halt: false, call: true  },
    CALLCODE     = 0xF2 { in: 7, out: 1, halt: false, call: true  },
    RETURN       = 0xF3 { in: 2, out: 0, halt: true,  call: false },
    DELEGATECALL = 0xF4 { in: 6, out: 1, halt: false, call: true  },
    CREATE2      = 0xF5 { in: 4, out: 1, halt: false, call: true  },
    STATICCALL   = 0xFA { in: 6, out: 1, halt: false, call: true  },
    REVERT       = 0xFD { in: 2, out: 0, halt: true,  call: false },
    SELFDESTRUCT = 0xFF { in: 1, out: 0, halt: true,  call: false },
}


// if not this then it would look smthng like thispub enum Opcode {
//     STOP = 0x00,
//     ADD = 0x01,
//     MUL = 0x02,
//     // ... 140+ more variants
// }

// impl Opcode {
//     pub fn from_u8(byte: u8) -> Self {
//         match byte {
//             0x00 => Opcode::STOP,
//             0x01 => Opcode::ADD,
//             0x02 => Opcode::MUL,
//             // ... 140+ more match arms
//         }
//     }
    
//     pub fn info(&self) -> OpcodeInfo {
//         match self {
//             Opcode::STOP => OpcodeInfo { name: "STOP", bytes: 0x00, inputs: 0, outputs: 0, is_halt: true, is_call: false },
//             Opcode::ADD => OpcodeInfo { name: "ADD", bytes: 0x01, inputs: 2, outputs: 1, is_halt: false, is_call: false },
//             // ... 140+ more match arms
//         }
//     }
// }