use anchor_lang::prelude::*;



#[error_code]
pub enum ExcrowError{
    #[msg("Escrow account already exists")]
    EscrowExists
}