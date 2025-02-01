use anchor_lang::prelude::*;
use solana_program::pubkey::PUBKEY_BYTES;
use spl_governance::state::token_owner_record;
use spl_governance::tools::spl_token::{get_spl_token_amount, get_spl_token_mint, get_spl_token_owner};

use crate::{
    error::NftVoterError,
    id,
    state::{CollectionConfig, VoterWeightRecord},
    tools::{
        anchor::DISCRIMINATOR_SIZE,
        token_metadata::get_token_metadata_for_mint,
    },
};

// ---------------------------------------------
// Structs and Accounts
// ---------------------------------------------

/// Registrar which stores NFT voting configuration for the given Realm
#[account]
#[derive(Debug, PartialEq)]
pub struct Registrar {
    /// spl-governance program the Realm belongs to
    pub governance_program_id: Pubkey,

    /// Realm of the Registrar
    pub realm: Pubkey,

    /// Governing token mint the Registrar is for
    pub governing_token_mint: Pubkey,

    /// MPL Collection used for voting
    pub collection_configs: Vec<CollectionConfig>,

    /// Reserved for future upgrades
    pub reserved: [u8; 128],
}

impl Registrar {
    /// Calculate the required space for the Registrar account
    pub fn get_space(max_collections: u8) -> usize {
        DISCRIMINATOR_SIZE
            + PUBKEY_BYTES * 3
            + 4
            + max_collections as usize * (PUBKEY_BYTES + 4 + 8 + 8)
            + 128
    }

    /// Retrieve collection configuration for a given collection
    pub fn get_collection_config(&self, collection: Pubkey) -> Result<&CollectionConfig> {
        self.collection_configs
            .iter()
            .find(|cc| cc.collection == collection)
            .ok_or_else(|| NftVoterError::CollectionNotFound.into())
    }
}

// ---------------------------------------------
// Helper Functions
// ---------------------------------------------

/// Returns Registrar PDA seeds
pub fn get_registrar_seeds<'a>(
    realm: &'a Pubkey,
    governing_token_mint: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [b"registrar", realm.as_ref(), governing_token_mint.as_ref()]
}

/// Returns Registrar PDA address
pub fn get_registrar_address(realm: &Pubkey, governing_token_mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&get_registrar_seeds(realm, governing_token_mint), &id()).0
}

/// Resolves governing_token_owner from the voter TokenOwnerRecord.
/// It validates the Registrar and VoterWeightRecord and ensures the signer is a valid delegate or owner.
pub fn resolve_governing_token_owner(
    registrar: &Registrar,
    voter_token_owner_record_info: &AccountInfo,
    voter_authority_info: &AccountInfo,
    voter_weight_record: &VoterWeightRecord,
) -> Result<Pubkey> {
    let voter_token_owner_record =
        token_owner_record::get_token_owner_record_data_for_realm_and_governing_mint(
            &registrar.governance_program_id,
            voter_token_owner_record_info,
            &registrar.realm,
            &registrar.governing_token_mint,
        )?;

    voter_token_owner_record.assert_token_owner_or_delegate_is_signer(voter_authority_info)?;

    // Ensure TokenOwnerRecord matches the VoterWeightRecord
    require_eq!(
        voter_token_owner_record.governing_token_owner,
        voter_weight_record.governing_token_owner,
        NftVoterError::InvalidTokenOwnerForVoterWeightRecord
    );

    Ok(voter_token_owner_record.governing_token_owner)
}

/// Resolves vote weight and voting mint for the given NFT
pub fn resolve_nft_vote_weight_and_mint(
    registrar: &Registrar,
    governing_token_owner: &Pubkey,
    nft_info: &AccountInfo,
    nft_metadata_info: &AccountInfo,
    unique_nft_mints: &mut Vec<Pubkey>,
) -> Result<(u64, Pubkey)> {
    let nft_owner = get_spl_token_owner(nft_info)?;

    // Ensure the governing token owns the NFT
    require!(
        nft_owner == *governing_token_owner,
        NftVoterError::VoterDoesNotOwnNft
    );

    let nft_mint = get_spl