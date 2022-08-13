pub mod context;
pub mod error;
pub mod processor;
pub mod state;
use context::{Initialize, Exchange, Cancel};
use anchor_lang::prelude::*;
use anchor_spl::token;
use spl_token::instruction::AuthorityType;


declare_id!("6cn9ZD4tPQjyo1iSaBqcuuCFnPhLj6naTCAtqkKKRmht");

#[program]
pub mod excrow {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"excrow";

    pub fn initialize(ctx: Context<Initialize>,        
        _vault_account_bump: u8,
        initializer_amount: u64,
        taker_amount: u64,) -> ProgramResult {
            // set initializer key in escrow contract account from passed instruction context
            ctx.accounts.escrow_account.initializer_key = *ctx.accounts.initializer.key;
            // set depositing account in escrow contract account from passed instruction context
            ctx.accounts.escrow_account.initializer_deposit_token_account = *ctx.accounts.initializer_deposit_token_account.to_account_info().key;
            // set receiving account in escrow contract account from passed instruction context
            ctx.accounts.escrow_account.initializer_receive_token_account = *ctx.accounts.initializer_receive_token_account.to_account_info().key;
            // set initializer amount
            ctx.accounts.escrow_account.initializer_amount = initializer_amount;    
            // set taker amount
            ctx.accounts.escrow_account.taker_amount = taker_amount;

            let (vault_authority, _vault_account_bump) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
            token::set_authority(ctx.accounts.into_set_authority_context(), AuthorityType::AccountOwner, Some(vault_authority));
        Ok(())
    }
    pub fn cancel(ctx: Context<Cancel>) -> ProgramResult {
        let (_vault_authority, vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let authority_seeds = &[&ESCROW_PDA_SEED[..], &[vault_authority_bump]];

        token::transfer(
            ctx.accounts
                .into_transfer_to_initializer_context()
                .with_signer(&[&authority_seeds[..]]),
            ctx.accounts.escrow_account.initializer_amount,
        )?;

        token::close_account(
            ctx.accounts
                .into_close_context()
                .with_signer(&[&authority_seeds[..]]),
        )?;

        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> ProgramResult {
        let (_vault_authority, vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let authority_seeds = &[&ESCROW_PDA_SEED[..], &[vault_authority_bump]];

        token::transfer(
            ctx.accounts.into_transfer_to_initializer_context(),
            ctx.accounts.escrow_account.taker_amount,
        )?;

        token::transfer(
            ctx.accounts
                .into_transfer_to_taker_context()
                .with_signer(&[&authority_seeds[..]]),
            ctx.accounts.escrow_account.initializer_amount,
        )?;

        token::close_account(
            ctx.accounts
                .into_close_context()
                .with_signer(&[&authority_seeds[..]]),
        )?;

        Ok(())
    }
}
