use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	Account,
	AccountPDA,
	GemMetadata,
};


/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` mint: [Mint] 
/// 2. `[writable]` gem: [GemMetadata] 
/// 3. `[writable]` account: [Account] The account to burn from.
/// 4. `[signer]` owner: [AccountInfo] The account's owner/delegate.
/// 5. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 6. `[]` token_program: [AccountInfo] SPL Token program
/// 7. `[writable]` assoc_token_account: [Account] The account to mint tokens to.
/// 8. `[]` csl_spl_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
///
/// Data:
/// - new_color: [String] 
/// - new_rarity: [String] 
/// - new_description: [String] 
/// - modify_owner: [Pubkey] 
pub fn modify(
	program_id: &Pubkey,
	for_burn: &[&AccountInfo],
	for_mint_to: &[&AccountInfo],
	mint: &Account<spl_token::state::Mint>,
	gem: &mut AccountPDA<GemMetadata>,
	account: &AccountPDA<spl_token::state::Account>,
	owner: &AccountInfo,
	wallet: &AccountInfo,
	assoc_token_account: &AccountPDA<spl_token::state::Account>,
	new_color: String,
	new_rarity: String,
	new_description: String,
	modify_owner: Pubkey,
) -> ProgramResult {
    // Verify that the program ID is correct
    
    // Check if the NFT is in the account
    // Add your business logic here
    // You can implement additional checks or restrictions on modifying NFTs

    // Burn the NFT using the SPL token program
    csl_spl_token::src::cpi::burn(for_burn, Default::default())?;

    // Mint the new token to the associated token account
    csl_spl_token::src::cpi::mint_to(for_mint_to, Default::default())?;

    // Update the Gem metadata with the new information
    gem.data.color = new_color;
    gem.data.rarity = new_rarity;
    gem.data.short_description = new_description;

    Ok(())
}