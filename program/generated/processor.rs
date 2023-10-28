// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use std::str::FromStr;
use borsh::BorshSerialize;
use solana_program::account_info::{AccountInfo, next_account_info, next_account_infos};
use solana_program::borsh0_10::try_from_slice_unchecked;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::{msg, system_program};
use solana_program::sysvar::Sysvar;
use solana_program::program_pack::Pack;
use crate::generated::errors::NftError;
use crate::generated::instructions::NftInstruction;

use crate::generated::state::{
	Account,
	AccountPDA,
	GemMetadata,
};
use crate::src::*;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        let instruction = NftInstruction::unpack(data)?;

        match instruction {
			NftInstruction::Mint(args) => {
				msg!("Instruction: Mint");
				Self::process_mint(
					program_id,
					accounts, 
					args.color,
					args.rarity,
					args.short_description,
					args.mint_owner,
				)
			}
			NftInstruction::Transfer(args) => {
				msg!("Instruction: Transfer");
				Self::process_transfer(
					program_id,
					accounts, 
					args.new_owner,
				)
			}
			NftInstruction::Burn(args) => {
				msg!("Instruction: Burn");
				Self::process_burn(
					program_id,
					accounts, 
					args.burn_owner,
				)
			}
			NftInstruction::Modify(args) => {
				msg!("Instruction: Modify");
				Self::process_modify(
					program_id,
					accounts, 
					args.new_color,
					args.new_rarity,
					args.new_description,
					args.modify_owner,
				)
			}
        }
    }

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
	pub fn process_mint(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		color: String,
		rarity: String,
		short_description: String,
		mint_owner: Pubkey,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let mint_info = next_account_info(account_info_iter)?;
		let gem_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;
		let funding_info = next_account_info(account_info_iter)?;
		let assoc_token_account_info = next_account_info(account_info_iter)?;
		let wallet_info = next_account_info(account_info_iter)?;
		let token_program_info = next_account_info(account_info_iter)?;
		let owner_info = next_account_info(account_info_iter)?;
		let csl_spl_token_v_0_0_0_info = next_account_info(account_info_iter)?;
		let csl_spl_assoc_token_v_0_0_0_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (gem_pubkey, gem_bump) = Pubkey::find_program_address(
			&[b"gem", mint_info.key.as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}

		if mint_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}

		if *gem_info.key != gem_pubkey {
			return Err(NftError::NotExpectedAddress.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::NotExpectedAddress.into());
		}

		if funding_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}

		if *token_program_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftError::NotExpectedAddress.into());
		}

		if owner_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}

		if *csl_spl_token_v_0_0_0_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftError::NotExpectedAddress.into());
		}

		if *csl_spl_assoc_token_v_0_0_0_info.key != Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap() {
			return Err(NftError::NotExpectedAddress.into());
		}


		// Accounts Initializations
		let space = spl_token::state::Mint::LEN;
		let rent = Rent::get()?;
		let rent_minimum_balance = rent.minimum_balance(space);

		invoke(
			&create_account(
				&fee_payer_info.key,
				&mint_info.key,
				rent_minimum_balance,
				space as u64,
				&Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
			),
			&[fee_payer_info.clone(), mint_info.clone()],
		)?;

		let space = GemMetadata::LEN;
		let rent = Rent::get()?;
		let rent_minimum_balance = rent.minimum_balance(space);

		invoke_signed(
			&create_account(
				&fee_payer_info.key,
				&gem_info.key,
				rent_minimum_balance,
				space as u64,
				program_id,
			),
			&[fee_payer_info.clone(), gem_info.clone()],
			&[&[b"gem", mint_info.key.as_ref(), &[gem_bump]]],
		)?;


		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if *mint_info.owner != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if *funding_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if *wallet_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if mint_info.data_len() != spl_token::state::Mint::LEN {
			return Err(NftError::InvalidAccountLen.into());
		}

		if gem_info.data_len() != GemMetadata::LEN {
			return Err(NftError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let mint = Account::new(
			&mint_info,
			spl_token::state::Mint::unpack_from_slice(&mint_info.data.borrow()).unwrap(),
		);
		let gem = &mut AccountPDA::new(
			&gem_info,
			try_from_slice_unchecked::<GemMetadata>(&gem_info.data.borrow()).unwrap(),
			gem_bump,
		);

		// Calling STUB
		mint::mint(
			program_id,
			&vec![
				mint_info,
			],
			&vec![
				funding_info,
				assoc_token_account_info,
				wallet_info,
				mint_info,
				system_program_info,
				token_program_info,
			],
			&vec![
				mint_info,
				assoc_token_account_info,
				owner_info,
				wallet_info,
				token_program_info,
			],
			&vec![
				mint_info,
				owner_info,
			],
			&mint,
			gem,
			funding_info,
			assoc_token_account_info,
			wallet_info,
			owner_info,
			color,
			rarity,
			short_description,
			mint_owner,
		)?;

		// Accounts Serialization
		gem.data.serialize(&mut &mut gem_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_transfer(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		new_owner: Pubkey,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let mint_info = next_account_info(account_info_iter)?;
		let gem_info = next_account_info(account_info_iter)?;
		let funding_info = next_account_info(account_info_iter)?;
		let assoc_token_account_info = next_account_info(account_info_iter)?;
		let wallet_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;
		let token_program_info = next_account_info(account_info_iter)?;
		let source_info = next_account_info(account_info_iter)?;
		let destination_info = next_account_info(account_info_iter)?;
		let authority_info = next_account_info(account_info_iter)?;
		let csl_spl_assoc_token_v_0_0_0_info = next_account_info(account_info_iter)?;
		let csl_spl_token_v_0_0_0_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (gem_pubkey, gem_bump) = Pubkey::find_program_address(
			&[b"gem", mint_info.key.as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}

		if *gem_info.key != gem_pubkey {
			return Err(NftError::NotExpectedAddress.into());
		}

		if funding_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::NotExpectedAddress.into());
		}

		if *token_program_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftError::NotExpectedAddress.into());
		}

		if authority_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}

		if *csl_spl_assoc_token_v_0_0_0_info.key != Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap() {
			return Err(NftError::NotExpectedAddress.into());
		}

		if *csl_spl_token_v_0_0_0_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftError::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if *mint_info.owner != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if *funding_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if *wallet_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if mint_info.data_len() != spl_token::state::Mint::LEN {
			return Err(NftError::InvalidAccountLen.into());
		}

		if gem_info.data_len() != GemMetadata::LEN {
			return Err(NftError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let mint = Account::new(
			&mint_info,
			spl_token::state::Mint::unpack_from_slice(&mint_info.data.borrow()).unwrap(),
		);
		let gem = &mut AccountPDA::new(
			&gem_info,
			try_from_slice_unchecked::<GemMetadata>(&gem_info.data.borrow()).unwrap(),
			gem_bump,
		);

		// Calling STUB
		transfer::transfer(
			program_id,
			&vec![
				funding_info,
				assoc_token_account_info,
				wallet_info,
				mint_info,
				system_program_info,
				token_program_info,
			],
			&vec![
				source_info,
				mint_info,
				destination_info,
				authority_info,
			],
			&mint,
			gem,
			funding_info,
			assoc_token_account_info,
			wallet_info,
			source_info,
			destination_info,
			authority_info,
			new_owner,
		)?;

		// Accounts Serialization
		gem.data.serialize(&mut &mut gem_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_burn(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		burn_owner: Pubkey,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let mint_info = next_account_info(account_info_iter)?;
		let gem_info = next_account_info(account_info_iter)?;
		let account_info = next_account_info(account_info_iter)?;
		let owner_info = next_account_info(account_info_iter)?;
		let wallet_info = next_account_info(account_info_iter)?;
		let token_program_info = next_account_info(account_info_iter)?;
		let csl_spl_token_v_0_0_0_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (gem_pubkey, gem_bump) = Pubkey::find_program_address(
			&[b"gem", mint_info.key.as_ref()],
			program_id,
		);
		let (account_pubkey, account_bump) = Pubkey::find_program_address(
			&[wallet_info.key.as_ref(), token_program_info.key.as_ref(), mint_info.key.as_ref()],
			&Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}

		if *gem_info.key != gem_pubkey {
			return Err(NftError::NotExpectedAddress.into());
		}

		if *account_info.key != account_pubkey {
			return Err(NftError::NotExpectedAddress.into());
		}

		if owner_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}

		if *token_program_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftError::NotExpectedAddress.into());
		}

		if *csl_spl_token_v_0_0_0_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftError::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if *mint_info.owner != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if *wallet_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if mint_info.data_len() != spl_token::state::Mint::LEN {
			return Err(NftError::InvalidAccountLen.into());
		}

		if gem_info.data_len() != GemMetadata::LEN {
			return Err(NftError::InvalidAccountLen.into());
		}

		if account_info.data_len() != spl_token::state::Account::LEN {
			return Err(NftError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let mint = Account::new(
			&mint_info,
			spl_token::state::Mint::unpack_from_slice(&mint_info.data.borrow()).unwrap(),
		);
		let gem = &mut AccountPDA::new(
			&gem_info,
			try_from_slice_unchecked::<GemMetadata>(&gem_info.data.borrow()).unwrap(),
			gem_bump,
		);
		let account = AccountPDA::new(
			&account_info,
			spl_token::state::Account::unpack_from_slice(&account_info.data.borrow()).unwrap(),
			account_bump,
		);

		// Calling STUB
		burn::burn(
			program_id,
			&vec![
				account_info,
				mint_info,
				owner_info,
				wallet_info,
				token_program_info,
			],
			&mint,
			gem,
			&account,
			owner_info,
			wallet_info,
			burn_owner,
		)?;

		// Accounts Serialization
		gem.data.serialize(&mut &mut gem_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_modify(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		new_color: String,
		new_rarity: String,
		new_description: String,
		modify_owner: Pubkey,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let mint_info = next_account_info(account_info_iter)?;
		let gem_info = next_account_info(account_info_iter)?;
		let account_info = next_account_info(account_info_iter)?;
		let owner_info = next_account_info(account_info_iter)?;
		let wallet_info = next_account_info(account_info_iter)?;
		let token_program_info = next_account_info(account_info_iter)?;
		let assoc_token_account_info = next_account_info(account_info_iter)?;
		let csl_spl_token_v_0_0_0_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (gem_pubkey, gem_bump) = Pubkey::find_program_address(
			&[b"gem", mint_info.key.as_ref()],
			program_id,
		);
		let (account_pubkey, account_bump) = Pubkey::find_program_address(
			&[wallet_info.key.as_ref(), token_program_info.key.as_ref(), mint_info.key.as_ref()],
			&Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
		);
		let (assoc_token_account_pubkey, assoc_token_account_bump) = Pubkey::find_program_address(
			&[wallet_info.key.as_ref(), token_program_info.key.as_ref(), mint_info.key.as_ref()],
			&Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}

		if *gem_info.key != gem_pubkey {
			return Err(NftError::NotExpectedAddress.into());
		}

		if *account_info.key != account_pubkey {
			return Err(NftError::NotExpectedAddress.into());
		}

		if owner_info.is_signer != true {
			return Err(NftError::InvalidSignerPermission.into());
		}

		if *token_program_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftError::NotExpectedAddress.into());
		}

		if *assoc_token_account_info.key != assoc_token_account_pubkey {
			return Err(NftError::NotExpectedAddress.into());
		}

		if *csl_spl_token_v_0_0_0_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftError::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if *mint_info.owner != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if *wallet_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(NftError::WrongAccountOwner.into());
		}

		if mint_info.data_len() != spl_token::state::Mint::LEN {
			return Err(NftError::InvalidAccountLen.into());
		}

		if gem_info.data_len() != GemMetadata::LEN {
			return Err(NftError::InvalidAccountLen.into());
		}

		if account_info.data_len() != spl_token::state::Account::LEN {
			return Err(NftError::InvalidAccountLen.into());
		}

		if assoc_token_account_info.data_len() != spl_token::state::Account::LEN {
			return Err(NftError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let mint = Account::new(
			&mint_info,
			spl_token::state::Mint::unpack_from_slice(&mint_info.data.borrow()).unwrap(),
		);
		let gem = &mut AccountPDA::new(
			&gem_info,
			try_from_slice_unchecked::<GemMetadata>(&gem_info.data.borrow()).unwrap(),
			gem_bump,
		);
		let account = AccountPDA::new(
			&account_info,
			spl_token::state::Account::unpack_from_slice(&account_info.data.borrow()).unwrap(),
			account_bump,
		);
		let assoc_token_account = AccountPDA::new(
			&assoc_token_account_info,
			spl_token::state::Account::unpack_from_slice(&assoc_token_account_info.data.borrow()).unwrap(),
			assoc_token_account_bump,
		);

		// Calling STUB
		modify::modify(
			program_id,
			&vec![
				account_info,
				mint_info,
				owner_info,
				wallet_info,
				token_program_info,
			],
			&vec![
				mint_info,
				assoc_token_account_info,
				owner_info,
				wallet_info,
				token_program_info,
			],
			&mint,
			gem,
			&account,
			owner_info,
			wallet_info,
			&assoc_token_account,
			new_color,
			new_rarity,
			new_description,
			modify_owner,
		)?;

		// Accounts Serialization
		gem.data.serialize(&mut &mut gem_info.data.borrow_mut()[..])?;
		
		Ok(())
	}
}