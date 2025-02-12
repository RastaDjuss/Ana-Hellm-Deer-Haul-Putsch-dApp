#![allow(clippy::result_large_err)]

mod build;

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod ana_chain {
  use super::*;

  // Fermer l'instance d'AnaChain
  pub fn close(_ctx: Context<CloseAnaChain>) -> Result<()> {
    Ok(())
  }

  // Décrémenter le compteur
  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.anachain.count = ctx
        .accounts
        .anachain
        .count
        .checked_sub(1)
        .ok_or_else(|| error!(ErrorCode::ArithmeticError))?;
    Ok(())
  }

  // Incrémenter le compteur
  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.anachain.count = ctx
        .accounts
        .ana_chain
        .count
        .checked_add(1)
        .ok_or_else(|| error!(ErrorCode::ArithmeticError))?;
    Ok(())
  }

  // Initialiser AnaChain
  pub fn initialize(ctx: Context<InitializeAnaChain>) -> Result<()> {
    let ana_chain = &mut ctx.accounts.ana_chain;
    ana_chain.count = 0; // Compteur initialisé à 0
    Ok(())
  }

  // Définir une valeur personnalisée pour le compteur
  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.anachain.count = value;
    Ok(())
  }

  // Exemple de gouvernance : ajouter une fonctionnalité pour restreindre les actions à un admin
  pub fn update_admin(ctx: Context<UpdateAdmin>, new_admin: Pubkey) -> Result<()> {
    let anachain = &mut ctx.accounts.anachain;
    require!(
            ctx.accounts.admin.key() == anachain.admin,
            ErrorCode::Unauthorized
        );
    anachain.admin = new_admin;
    Ok(())
  }
}
#[derive(Accounts)]
pub struct InitializeAnachain<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
        init,
        space = 8 + Anachain::INIT_SPACE,
        payer = payer
  )]
  pub anachain: Account<'info, Anachain>,
  pub system_program: Program<'info, System>,
}
pub struct CloseAnachain<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
        mut,
        close = payer, // fermeture du compte et retour des lamports au payer
  )]
  pub anachain: Account<'info, Anachain>,
}

pub struct Update<'info> {
  #[account(mut)]
  pub anachain: Account<'info, Anachain>,
}
// Contexte pour mettre à jour l'administrateur
pub struct UpdateAdmin<'info> {
  #[account(mut)]
  pub anachain: Account<'info, Anachain>,
  pub admin: Signer<'info>, // L'administrateur actuel doit signer la transaction
}

#[account]
#[derive(InitSpace)]
pub struct Anachain {
  pub count: u8,        // Le compteur principal
  pub admin: Pubkey,    // Adresse publique de l'administrateur
}

#[error_code]
pub enum ErrorCode {
  #[msg("Arithmetic operation error")]
  ArithmeticError,
  #[msg("You are not authorized to perform this action")]
  Unauthorized,
}