use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum CounterInstructions {
    Increment(u32),
    Decrement(u32),
}

impl CounterInstructions {
    /// Wire format (Borsh): 1 byte variant (0 = Increment, 1 = Decrement) + u32 little-endian amount.
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpacks_increment_and_decrement() {
        assert_eq!(
            CounterInstructions::unpack(&[0, 5, 0, 0, 0]).unwrap(),
            CounterInstructions::Increment(5)
        );
        assert_eq!(
            CounterInstructions::unpack(&[1, 0, 1, 0, 0]).unwrap(),
            CounterInstructions::Decrement(256)
        );
    }

    #[test]
    fn rejects_bad_input() {
        assert!(CounterInstructions::unpack(&[]).is_err());
        assert!(CounterInstructions::unpack(&[0, 1, 2]).is_err());
        assert!(CounterInstructions::unpack(&[2, 0, 0, 0, 0]).is_err());
        assert!(CounterInstructions::unpack(&[0, 1, 0, 0, 0, 9]).is_err());
    }

    #[test]
    fn roundtrips_with_borsh() {
        let bytes = CounterInstructions::Decrement(42).try_to_vec().unwrap();
        assert_eq!(
            CounterInstructions::unpack(&bytes).unwrap(),
            CounterInstructions::Decrement(42)
        );
    }
}
