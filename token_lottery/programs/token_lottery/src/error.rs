use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Lottery has not opened yet")]
    LotteryNotOpen,
    #[msg("Lottery has not completed yet")]
    LotteryNotCompleted,
    #[msg("User not authorised")]
    NotAuthorised,
    #[msg("Randomness has been already revealed")]
    RandomnessAlreadyRevealed,
    #[msg("Incorrect randomness account")]
    IncorrectRandomnessAccount,
    #[msg("Winner already chosen")]
    WinnerChosen,
    #[msg("Winner not chosen")]
    WinnerNotChosen,
    #[msg("Randomness not resolved")]
    RandomnessNotResolved,
    #[msg("Collection not verified")]
    CollectionNotVerified,
    #[msg("Incorrect collection")]
    IncorrectCollection,
    #[msg("Incorrect ticket")]
    IncorrectTicket,
    #[msg("No ticket")]
    NoTicket,
}
