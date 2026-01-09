use serde::{Serialize , Deserialize};
use std::fmt;
use alloy_primitives::U256;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Word(pub U256);

impl Word {
    pub const ZERO: Word = Word(U256::ZERO);

    pub fn from_u64(a : u64)-> Self {
        Self(U256::from(a))
    }
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result{
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result{
        write!(f, "{}", self.0)
    }
}
