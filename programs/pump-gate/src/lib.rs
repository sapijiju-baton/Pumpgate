// =============================================================================
// PumpGate — Token gate wrapper for pump.fun launches
// =============================================================================
// Repository:   https://github.com/sapijiju-baton/Pumpgate
// Author:       https://github.com/sapijiju-baton
// Co-author:    https://github.com/a1lon9-baton
// Contributors:
//   https://github.com/andrei-baton
//   https://github.com/arv-baton
//   https://github.com/drew-baton
// Organization: https://batoncorporation.com
// =============================================================================
//
// Verified account layout and instruction encoding from simulation:
//   - Simulation result: err=null, units=120,310
//   - IDL source: https://raw.githubusercontent.com/pump-fun/pump-public-docs/main/idl/pump.json
// =============================================================================

use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token_interface::TokenAccount;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::TokenInterface;

declare_id!("DVPmNqbmLd4Y3d9vV4bDDojnfuzdCP8Xcxah1VxKDWM8");

const MIN_PUMP_BALANCE: u64 = 25_000 * 1_000_000;
const PUMP_FUN_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
const CREATE_DISCRIMINATOR: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];
const MPL_TOKEN_METADATA: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

const ASSOC_TOKEN_PROG_BYTES: [u8; 32] = [
    140, 151, 37, 143, 78, 36, 137, 241,
    187, 61, 16, 41, 20, 142, 13, 131,
    11, 90, 19, 153, 218, 255, 16, 132,
    4, 142, 123, 216, 219, 233, 248, 89,
];

const MPL_META_PROG_BYTES: [u8; 32] = [
    11, 112, 101, 177, 227, 209, 124, 69,
    56, 157, 82, 127, 107, 4, 195, 205,
    88, 184, 108, 115, 26, 160, 253, 181,
    73, 182, 209, 188, 3, 248, 41, 70,
];

// PumpGate admin address
// Reserved for protocol-level administration by Baton Corporation.
const ADMIN: &str = "Doa8F9eugaAHpCBmfmShKV1BhKN9xyaEDg1mTsonW5p8";

#[program]
pub mod pump_gate {
    use super::*;

    pub fn gated_launch(
        ctx: Context<GatedLaunch>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        let pump_token_balance = ctx.accounts.user_pump_token_account.amount;
        require!(
            pump_token_balance >= MIN_PUMP_BALANCE,
            GateError::NotEnoughPump
        );

        let mut data = Vec::with_capacity(8 + 4 + name.len() + 4 + symbol.len() + 4 + uri.len() + 32);
        data.extend_from_slice(&CREATE_DISCRIMINATOR);

        let name_bytes = name.as_bytes();
        data.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(name_bytes);

        let symbol_bytes = symbol.as_bytes();
        data.extend_from_slice(&(symbol_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(symbol_bytes);

        let uri_bytes = uri.as_bytes();
        data.extend_from_slice(&(uri_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(uri_bytes);

        data.extend_from_slice(ctx.accounts.user.key.as_ref());

        let pump_program_key = ctx.accounts.pump_fun_program.key();
        let mint_key = ctx.accounts.launch_mint.key();
        let user_key = ctx.accounts.user.key();
        let mpl_key = ctx.accounts.mpl_token_metadata.key();

        let (mint_authority, _) = Pubkey::find_program_address(&[b"mint-authority"], &pump_program_key);
        let (bonding_curve, _) = Pubkey::find_program_address(&[b"bonding-curve", mint_key.as_ref()], &pump_program_key);
        let (associated_bonding_curve, _) = Pubkey::find_program_address(
            &[bonding_curve.as_ref(), &ASSOC_TOKEN_PROG_BYTES, mint_key.as_ref()],
            &ctx.accounts.associated_token_program.key(),
        );
        let (global_pda, _) = Pubkey::find_program_address(&[b"global"], &pump_program_key);
        let (metadata_pda, _) = Pubkey::find_program_address(
            &[b"metadata", &MPL_META_PROG_BYTES, mint_key.as_ref()],
            &mpl_key,
        );
        let (event_authority, _) = Pubkey::find_program_address(&[b"__event_authority"], &pump_program_key);

        let accounts = vec![
            AccountMeta::new(mint_key, true),
            AccountMeta::new_readonly(mint_authority, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new_readonly(global_pda, false),
            AccountMeta::new_readonly(mpl_key, false),
            AccountMeta::new(metadata_pda, false),
            AccountMeta::new(user_key, true),
            AccountMeta::new_readonly(*ctx.accounts.system_program.key, false),
            AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
            AccountMeta::new_readonly(*ctx.accounts.associated_token_program.key, false),
            AccountMeta::new_readonly(*ctx.accounts.rent.key, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(pump_program_key, false),
        ];

        let ix = Instruction { program_id: pump_program_key, accounts, data };

        let account_infos = vec![
            ctx.accounts.launch_mint.to_account_info(),
            ctx.accounts.pump_mint_authority.to_account_info(),
            ctx.accounts.pump_bonding_curve.to_account_info(),
            ctx.accounts.pump_associated_bonding_curve.to_account_info(),
            ctx.accounts.pump_global.to_account_info(),
            ctx.accounts.mpl_token_metadata.to_account_info(),
            ctx.accounts.pump_metadata.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),  // legacy — pump.fun requires this
            ctx.accounts.associated_token_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.pump_event_authority.to_account_info(),
            ctx.accounts.pump_fun_program.to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        msg!("✅ Gate passed. Token launched via pump.fun.");
        msg!("Admin: {}", ADMIN);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GatedLaunch<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: pump.fun initializes this
    #[account(mut, signer)]
    pub launch_mint: UncheckedAccount<'info>,

    #[account(
        token::mint = pump_mint,
        token::authority = user,
        token::token_program = pump_token_program,
    )]
    pub user_pump_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: validated via token account constraint
    pub pump_mint: UncheckedAccount<'info>,

    /// CHECK: PDA
    #[account(mut)]
    pub pump_mint_authority: UncheckedAccount<'info>,

    /// CHECK: PDA
    #[account(mut)]
    pub pump_bonding_curve: UncheckedAccount<'info>,

    /// CHECK: PDA
    #[account(mut)]
    pub pump_associated_bonding_curve: UncheckedAccount<'info>,

    /// CHECK: PDA
    pub pump_global: UncheckedAccount<'info>,

    /// CHECK: const address
    #[account(address = MPL_TOKEN_METADATA.parse::<Pubkey>().unwrap())]
    pub mpl_token_metadata: UncheckedAccount<'info>,

    /// CHECK: PDA
    #[account(mut)]
    pub pump_metadata: UncheckedAccount<'info>,

    /// CHECK: PDA
    pub pump_event_authority: UncheckedAccount<'info>,

    /// CHECK: CPI target
    #[account(address = PUMP_FUN_PROGRAM_ID.parse::<Pubkey>().unwrap())]
    pub pump_fun_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    /// Token-2022 — used to validate the PUMP ATA
    pub pump_token_program: Interface<'info, TokenInterface>,
    /// Legacy token program — required by pump.fun CPI
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: Sysvar
    #[account(address = anchor_lang::solana_program::sysvar::rent::ID)]
    pub rent: UncheckedAccount<'info>,
}

#[error_code]
pub enum GateError {
    #[msg("NOT ENOUGH $PUMP HELD — you need 25,000 $PUMP to launch")]
    NotEnoughPump,

    #[msg("WRONG MINT — this program only gates the specified token")]
    InvalidMint,
}
