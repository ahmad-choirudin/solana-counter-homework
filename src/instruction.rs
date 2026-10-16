use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

// Langkah 1: Tambahkan argumen u32 pada enum CounterInstructions
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstructions {
    Increment(u32),
    Decrement(u32),
}

// Langkah 2: Fungsi untuk unpack byte data instruksi dari client
pub fn unpack(input: &[u8]) -> Result<CounterInstructions, ProgramError> {
    // Ambil byte pertama sebagai penentu instruksi (variant)
    let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    // Proses unpack berdasarkan varian instruksi
    match variant {
        0 => {
            // Ambil 4 byte berikutnya untuk nilai u32 pada Increment
            let amount = rest
                .get(..4)
                .and_then(|slice| slice.try_into().ok())
                .map(u32::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok(CounterInstructions::Increment(amount))
        }
        1 => {
            // Ambil 4 byte berikutnya untuk nilai u32 pada Decrement
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
