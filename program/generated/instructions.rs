// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::generated::errors::NftError;

#[derive(BorshSerialize, Debug)]
pub enum NftInstruction {
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable, signer]` mint: [Mint] 
/// 2. `[writable]` gem: [GemMetadata] 
/// 3. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
/// 4. `[writable, signer]` funding: [AccountInfo] Funding account (must be a system account)
/// 5. `[writable]` assoc_token_account: [AccountInfo] Associated token account address to be created
/// 6. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 7. `[]` token_program: [AccountInfo] SPL Token program
/// 8. `[signer]` owner: [AccountInfo] The mint's minting authority.
/// 9. `[]` csl_spl_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
/// 10. `[]` csl_spl_assoc_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplAssocTokenProgram v0.0.0
///
/// Data:
/// - color: [String] 
/// - rarity: [String] 
/// - short_description: [String] 
/// - mint_owner: [Pubkey] 
	Mint(MintArgs),

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[]` mint: [Mint] 
/// 2. `[writable]` gem: [GemMetadata] 
/// 3. `[writable, signer]` funding: [AccountInfo] Funding account (must be a system account)
/// 4. `[writable]` assoc_token_account: [AccountInfo] Associated token account address to be created
/// 5. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 6. `[]` system_program: [AccountInfo] System program
/// 7. `[]` token_program: [AccountInfo] SPL Token program
/// 8. `[writable]` source: [AccountInfo] The source account.
/// 9. `[writable]` destination: [AccountInfo] The destination account.
/// 10. `[signer]` authority: [AccountInfo] The source account's owner/delegate.
/// 11. `[]` csl_spl_assoc_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplAssocTokenProgram v0.0.0
/// 12. `[]` csl_spl_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
///
/// Data:
/// - new_owner: [Pubkey] 
	Transfer(TransferArgs),

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` mint: [Mint] 
/// 2. `[writable]` gem: [GemMetadata] 
/// 3. `[writable]` account: [Account] The account to burn from.
/// 4. `[signer]` owner: [AccountInfo] The account's owner/delegate.
/// 5. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 6. `[]` token_program: [AccountInfo] SPL Token program
/// 7. `[]` csl_spl_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
///
/// Data:
/// - burn_owner: [Pubkey] 
	Burn(BurnArgs),

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
	Modify(ModifyArgs),

}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MintArgs {
	pub color: String,
	pub rarity: String,
	pub short_description: String,
	pub mint_owner: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TransferArgs {
	pub new_owner: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BurnArgs {
	pub burn_owner: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ModifyArgs {
	pub new_color: String,
	pub new_rarity: String,
	pub new_description: String,
	pub modify_owner: Pubkey,
}

impl NftInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(NftError::InvalidInstruction)?;

        Ok(match variant {
			0 => Self::Mint(MintArgs::try_from_slice(rest).unwrap()),
			1 => Self::Transfer(TransferArgs::try_from_slice(rest).unwrap()),
			2 => Self::Burn(BurnArgs::try_from_slice(rest).unwrap()),
			3 => Self::Modify(ModifyArgs::try_from_slice(rest).unwrap()),
			_ => return Err(NftError::InvalidInstruction.into())
        })
    }
}