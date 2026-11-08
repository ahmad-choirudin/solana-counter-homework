use borsh::{BorshDeserialize, BorshSerialize};

/// Counter account data. An all-zero account is a valid counter with `count == 0`.
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct CounterState {
    pub count: u64,
}

impl CounterState {
    /// Bytes the counter account must be allocated with.
    pub const SIZE: usize = 8;
}
