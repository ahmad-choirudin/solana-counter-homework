use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{instruction::CounterInstructions, state::CounterState};

#[repr(u32)]
pub enum CounterError {
    Underflow = 0,
}

impl From<CounterError> for ProgramError {
    fn from(e: CounterError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

pub struct Processor;

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = CounterInstructions::unpack(input)?;

        let account = next_account_info(&mut accounts.iter())?;

        if account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
        if !account.is_writable {
            return Err(ProgramError::InvalidArgument);
        }

        let mut data = account.try_borrow_mut_data()?;
        if data.len() < CounterState::SIZE {
            return Err(ProgramError::AccountDataTooSmall);
        }

        let mut state = CounterState::deserialize(&mut &data[..])
            .map_err(|_| ProgramError::InvalidAccountData)?;

        state.count = match instruction {
            CounterInstructions::Increment(n) => state
                .count
                .checked_add(u64::from(n))
                .ok_or(ProgramError::ArithmeticOverflow)?,
            CounterInstructions::Decrement(n) => state
                .count
                .checked_sub(u64::from(n))
                .ok_or(CounterError::Underflow)?,
        };

        state
            .serialize(&mut &mut data[..])
            .map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(owner_ok: bool, writable: bool, start: u64, ix: CounterInstructions) -> (ProgramResult, u64) {
        let program_id = Pubkey::new_unique();
        let key = Pubkey::new_unique();
        let other = Pubkey::new_unique();
        let owner = if owner_ok { program_id } else { other };
        let mut lamports = 0u64;
        let mut data = start.to_le_bytes().to_vec();
        let account = AccountInfo::new(
            &key, false, writable, &mut lamports, &mut data, &owner, false, 0,
        );

        let result = Processor::process(&program_id, &[account.clone()], &ix.try_to_vec().unwrap());
        let count = CounterState::deserialize(&mut &account.data.borrow()[..]).unwrap().count;
        (result, count)
    }

    #[test]
    fn increments_and_decrements() {
        assert_eq!(run(true, true, 10, CounterInstructions::Increment(5)).1, 15);
        assert_eq!(run(true, true, 10, CounterInstructions::Decrement(4)).1, 6);
    }

    #[test]
    fn rejects_underflow_and_overflow() {
        let (res, count) = run(true, true, 3, CounterInstructions::Decrement(4));
        assert_eq!(res, Err(ProgramError::Custom(CounterError::Underflow as u32)));
        assert_eq!(count, 3);

        let (res, _) = run(true, true, u64::MAX, CounterInstructions::Increment(1));
        assert_eq!(res, Err(ProgramError::ArithmeticOverflow));
    }

    #[test]
    fn rejects_wrong_owner_and_readonly_account() {
        assert_eq!(
            run(false, true, 0, CounterInstructions::Increment(1)).0,
            Err(ProgramError::IncorrectProgramId)
        );
        assert_eq!(
            run(true, false, 0, CounterInstructions::Increment(1)).0,
            Err(ProgramError::InvalidArgument)
        );
    }
}
