use num_enum::IntoPrimitive;
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum OreError {
    #[error("The epoch has ended and needs reset")]
    NeedsReset = 0,
    #[error("The provided hash is invalid")]
    HashInvalid = 1,
    #[error("The provided hash did not satisfy the minimum required difficulty")]
    HashTooEasy = 2,
    #[error("The claim amount cannot be greater than the claimable rewards")]
    ClaimTooLarge = 3,
    #[error("The clock time is invalid")]
    ClockInvalid = 4,
    #[error("You are trying to submit too soon")]
    Spam = 5,
    #[error("Only one hash may be validated per transaction")]
    TransactionInvalid = 6,
    #[error("The tolerance cannot exceed i64 max value")]
    ToleranceOverflow = 7,
    #[error("The maximum supply has been reached")]
    MaxSupply = 8,
    #[error("This account cannot be closed because it's the top staker")]
    CannotClose = 9,
    #[error("This account cannot be crowned because its last stake was too recent")]
    CannotCrown = 10,
}

impl From<OreError> for ProgramError {
    fn from(e: OreError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
