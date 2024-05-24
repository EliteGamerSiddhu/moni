use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    
    #[error("InvalidUnitPrice")]
    InvalidUnitPrice {},

    #[error("InvalidMaxTokens")]
    InvalidMaxTokens {},

    #[error("InvalidTokenReplyId")]
    InvalidTokenReplyId {},

    #[error("Cw721AlreadyLinked")]
    Cw721AlreadyLinked {},

    #[error("SoldOut")]
    SoldOut {},

    #[error("UnauthorizedTokenContract")]
    UnauthorizedTokenContract {},

    #[error("Uninitialized")]
    Uninitialized {},

    #[error("WrongPaymentAmount")]
    WrongPaymentAmount {},

    #[error("Cw721NotLinked")]
    Cw721NotLinked {},
}
