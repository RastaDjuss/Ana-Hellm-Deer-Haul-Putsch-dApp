#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

// Programme public ID
declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod ana_chain {
    use super::*;

    pub fn close(_ctx: Context<crate::CloseAnachain>) -> Result<()> {
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.anachain.count = ctx.accounts.anachain.count
            .checked_sub(1)
            .ok_or(ErrorCode::Underflow)?;
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.anachain.count = ctx.accounts.anachain.count
            .checked_add(1)
            .unwrap();
        Ok(())
    }

    pub fn initialize(ctx: Context<InitializeAnaChain>) -> Result<()> {
        ctx.accounts.anachain.count = 0;
        Ok(())
    }

    pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
        ctx.accounts.anachain.count = value;
        Ok(())
    }
}

// Déclaration du compte principal
#[account]
pub struct AnaChainState { // Renommé pour éviter tout conflit avec la variable
    pub count: u8, // Unique champ
}

// Initialisation avec un contexte
#[derive(Accounts)]
pub struct InitializeAnaChain<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = 9, // 8 octets + 1 pour le champ
        payer = payer
    )]
    pub anachain: Account<'info, AnaChainState>, // Utilise la nouvelle structure
    pub system_program: Program<'info, System>,
}

// Fermeture du compte
#[derive(Accounts)]
pub struct CloseAnachain<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        close = payer,
    )]
    pub anachain: Account<'info, AnaChainState>, // Évite conflits
}

// Mise à jour
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub anachain: Account<'info, AnaChainState>,
}

// Gestion des erreurs
#[error_code]
pub enum ErrorCode {
    #[msg("Cannot decrement. Count is at 0.")]
    Underflow,
}
