use solana_program::program_error::ProgramError;
use thiserror::Error;


#[derive(Error, Debug, Copy, Clone)]
pub enum AppletError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Rent Exempt")]
    NotRentExempt,
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,
    #[error("Amount Overflow")]
    AmountOverflow,
}

impl From<AppletError> for ProgramError {
    fn from(e: AppletError) -> Self {
        ProgramError::Custom(e as u32)
    }
}