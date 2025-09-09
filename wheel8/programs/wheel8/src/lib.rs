use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak;

declare_id!("216ZrFHfgxufofhxzxZ6QmV6geqHp1ZZUYheD5Mo4Amb"); // <-- update this!

#[program]
pub mod wheel8 {
    use super::*;

    /// Initialize the wheel configuration with 8 multipliers.
    pub fn initialize(ctx: Context<Initialize>, multipliers: [u16; 8]) -> Result<()> {
        ctx.accounts.config.multipliers = multipliers;
        Ok(())
    }

    /// Spin the wheel. Uses a simple keccak hash of (player_pubkey || unix_ts || client_seed)
    /// to pick an index 0..=7. Emits a SpinResult event.
    pub fn spin(ctx: Context<Spin>, client_seed: u64) -> Result<()> {
        let player = ctx.accounts.player.key();
        let ts = Clock::get()?.unix_timestamp as u64;

        // Build the entropy buffer.
        let mut buf = Vec::with_capacity(32 + 8 + 8);
        buf.extend_from_slice(&player.to_bytes());
        buf.extend_from_slice(&ts.to_le_bytes());
        buf.extend_from_slice(&client_seed.to_le_bytes());

        // Hash and pick an index in [0..8)
        let hash = keccak::hash(&buf);
        let idx = (hash.0[0] % 8) as u8;

        let mult = ctx.accounts.config.multipliers[idx as usize];

        emit!(SpinResult {
            player,
            index: idx,
            multiplier: mult,
            rand: hash.0,
        });

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

#[derive(Accounts)]
pub struct Spin<'info> {
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub player: Signer<'info>,
    // system_program not required for spin in this stage
}

#[account]
pub struct Config {
    pub multipliers: [u16; 8], // 16 bytes
}
impl Config {
    // 8 bytes account discriminator + 16 bytes multipliers
    pub const LEN: usize = 8 + (8 * 2);
}

#[event]
pub struct SpinResult {
    pub player: Pubkey,
    pub index: u8,
    pub multiplier: u16,
    pub rand: [u8; 32],
}
