use serde::{Serialize , Deserialize};
use std::fmt;
use alloy_primitives::U256;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(transaparent)]
pub struct word(pub U256);

impl Word {
    pub const ZERO: Word = Word(U256::Zero);

    pub fn from_u64(a : u64)-> Self {
        Self(U256::from_u64(a))
    }
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result{
        write!(f, {}, self.0)
    }
}

impl fmt::Display for word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result{
        write!(f, {}. self.0)
    }
}
