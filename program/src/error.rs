// program specific errors
use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Rent Exempt")]
    NotRentExempt,
}

// defines conversion from our custom error to ProgramError
impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        // ProgramError has a custom variation
        ProgramError::Custom(e as u32)
    }
}
