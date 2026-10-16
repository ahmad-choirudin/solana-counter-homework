
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstructions {
    Increment(u32),
    Decrement(u32),
}

pub fn unpack(input: &[u8]) -> Result<CounterInstructions, ProgramError> {
    let (&variant, rest) = input
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match variant {
        0 => {
            let amount = rest
                .get(..4)
                .and_then(|slice| slice.try_into().ok())
                .map(u32::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;

            Ok(CounterInstructions::Increment(amount))
        }
        1 => {
            let amount = rest
                .get(..4)
                .and_then(|slice| slice.try_into().ok())
                .map(u32::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;

            Ok(CounterInstructions::Decrement(amount))
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
