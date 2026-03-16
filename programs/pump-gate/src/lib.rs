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

use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token::TokenAccount;

declare_id!("REPLACE_WITH_YOUR_PROGRAM_ID");

// Minimum PUMP tokens required (25,000 with 6 decimals)
const MIN_PUMP_BALANCE: u64 = 25_000 * 1_000_000;

// The one and only mint that can be launched through this gate
// Pre-derive this before deploying — only this token can pass
const ALLOWED_MINT: &str = "3sDpTMXy5LgoPKCL1ws86ohfVqNaQ2pkYTeoTCVz9XFj";

// pump.fun program ID
const PUMP_FUN_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

#[program]
pub mod pump_gate {
    use super::*;

    pub fn gated_launch(
        ctx: Context<GatedLaunch>,
        pump_ix_data: Vec<u8>,
    ) -> Result<()> {
        // --- CHECK 1: Must hold 25,000 PUMP ---
        let pump_token_balance = ctx.accounts.user_pump_token_account.amount;
        require!(
            pump_token_balance >= MIN_PUMP_BALANCE,
            GateError::NotEnoughPump
        );

        // --- CHECK 2: Must be launching the correct mint ---
        let allowed_mint = ALLOWED_MINT.parse::<Pubkey>()
            .map_err(|_| GateError::InvalidMint)?;

        require!(
            ctx.accounts.launch_mint.key() == allowed_mint,
            GateError::InvalidMint
        );

        // --- FORWARD TO PUMP.FUN ---
        let account_metas: Vec<anchor_lang::solana_program::instruction::AccountMeta> = ctx
            .remaining_accounts
            .iter()
            .map(|a| {
                if a.is_writable {
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        *a.key,
                        a.is_signer,
                    )
                } else {
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        *a.key,
                        a.is_signer,
                    )
                }
            })
            .collect();

        let ix = Instruction {
            program_id: ctx.accounts.pump_fun_program.key(),
            accounts: account_metas,
            data: pump_ix_data,
        };

        invoke_signed(
            &ix,
            ctx.remaining_accounts,
            &[],
        )?;

        msg!("✅ Gate passed. Launch forwarded to pump.fun.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GatedLaunch<'info> {
    /// The wallet trying to launch
    #[account(mut)]
    pub user: Signer<'info>,

    /// User's PUMP token account
    #[account(
        associated_token::mint = pump_mint,
        associated_token::authority = user,
    )]
    pub user_pump_token_account: Account<'info, TokenAccount>,

    /// PUMP token mint
    /// CHECK: Just used to validate the token account
    pub pump_mint: UncheckedAccount<'info>,

    /// The mint of the token being launched — must match ALLOWED_MINT
    /// CHECK: We check this against the hardcoded CA
    pub launch_mint: UncheckedAccount<'info>,

    /// pump.fun program
    /// CHECK: CPI target
    pub pump_fun_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum GateError {
    #[msg("NOT ENOUGH $PUMP HELD — you need 25,000 $PUMP to launch")]
    NotEnoughPump,

    #[msg("WRONG MINT — this program only launches one specific token")]
    InvalidMint,
}
