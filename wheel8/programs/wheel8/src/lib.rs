use anchor_lang::prelude::*;
// use anchor_lang::solana_program::hash::hashv; // <-- use SHA-256 hashv here


declare_id!("5SZsSFXqN7yBAT6yJWHuBNheyDm7QL9mdqoBUvU5ukAG"); // replace with your program id before deploy

#[program]
pub mod wheel8 {
    use super::*;

    /// Initialize the wheel configuration with 8 multipliers.
    pub fn initialize(ctx: Context<Initialize>, multipliers: [u16; 8]) -> Result<()> {
        ctx.accounts.config.multipliers = multipliers;
        Ok(())
    }

    /// Spin the wheel: keccak(player_pubkey || unix_ts || client_seed).
    /// Emits a SpinResult event.
    pub fn spin(ctx: Context<Spin>, seed: u64) -> Result<()> {
        // simple on-chain PRNG: xorshift64*
        let clock = Clock::get()?;                 // brings in the current slot
        let mut x = seed ^ clock.slot;             // mix user seed with slot
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        let rnd = x.wrapping_mul(2685821657736338717);

        let outcome: u8 = (rnd % 8) as u8;
        let payout: u64 = (outcome as u64) * 10;

        msg!("🌀 spin: seed={}, slot={}, rnd={}", seed, clock.slot, rnd);
        msg!("🎯 outcome={}, payout={}", outcome, payout);

        // emit!(SpinResult { ... }) if you added the event
        Ok(())
    }

}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = Config::LEN)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Your existing accounts
#[derive(Accounts)]
pub struct Spin<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
}


#[account]
pub struct Config {
    pub multipliers: [u16; 8],
}

impl Config {
    // 8 bytes discriminator + 8*2 bytes multipliers
    pub const LEN: usize = 8 + (8 * 2);
}

#[event]
pub struct SpinResult {
    pub player: Pubkey,
    pub index: u8,
    pub multiplier: u16,
    pub rand: [u8; 32],
}
