use anchor_lang::prelude::*;

// Programm‑ID
declare_id!("AYLD1111111111111111111111111111111111");

#[program]
pub mod autoyield {
    use super::*;

    // Initialisiert die globale Registry für Strategien
    pub fn init_registry(ctx: Context<InitRegistry>) -> Result<()> {
        let reg = &mut ctx.accounts.registry;
        reg.strategies = Vec::new();
        Ok(())
    }

    // Fügt eine neue Strategie‑PDA hinzu
    pub fn add_strategy(
        ctx: Context<AddStrategy>,
        strategy_key: Pubkey,
    ) -> Result<()> {
        let reg = &mut ctx.accounts.registry;
        require!(
            !reg.strategies.contains(&strategy_key),
            ErrorCode::StrategyAlreadyAdded
        );
        reg.strategies.push(strategy_key);
        Ok(())
    }

    // User hinterlegt SPL‑Token in einer Strategie
    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
    ) -> Result<()> {
        // SPL‑Token‑Transfer vom User zum Strategy‑Vault
        token::transfer(
            ctx.accounts
                .transfer_context()
                .with_signer(&[]),
            amount,
        )?;
        // User‑Position updaten (Zero‑Copy)
        let pos = &mut ctx.accounts.user_pos;
        pos.amount = pos.amount.checked_add(amount).unwrap();
        Ok(())
    }

    // Erntet für alle registrierten Strategien – parallel per CPI
    pub fn harvest_all(ctx: Context<HarvestAll>) -> Result<()> {
        let reg = &ctx.accounts.registry;
        for strat_key in reg.strategies.iter() {
            // CPI zu jeder Strategy‑PDA: strat.harvest()
            let seeds = [b"strategy", strat_key.as_ref(), &[ctx.bumps["registry"]]];
            let cpi_prog = ctx.accounts.strategy_program.to_account_info();
            let cpi_accounts = Harvest { /* ... */ };
            let cpi_ctx = CpiContext::new_with_signer(cpi_prog, cpi_accounts, &[&seeds]);
            strategy::cpi::harvest(cpi_ctx)?;
        }
        Ok(())
    }
}

// Accounts

#[account(zero_copy)]
pub struct Registry {
    // Liste aller Strategy‑Program­IDs
    pub strategies: Vec<Pubkey>,
}

#[derive(Accounts)]
pub struct InitRegistry<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 4 + (32 * 10)  // initialer Vektor‑Slot für bis zu 10 Strategien
    )]
    pub registry: AccountLoader<'info, Registry>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddStrategy<'info> {
    #[account(mut, has_one = authority)]
    pub registry: AccountLoader<'info, Registry>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 8,  // u64 für die Menge
        seeds = [b"userpos", user.key().as_ref(), strategy.key().as_ref()],
        bump
    )]
    pub user_pos: Account<'info, UserPosition>,
    /// CHECK: wird dynamisch über Registry gezogen
    pub strategy: UncheckedAccount<'info>,
    #[account(mut)]
    pub vault: UncheckedAccount<'info>, // SPL‑Token‑Vault der Strategie
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserPosition {
    pub amount: u64,
}

#[derive(Accounts)]
pub struct HarvestAll<'info> {
    #[account(has_one = authority)]
    pub registry: AccountLoader<'info, Registry>,
    pub authority: Signer<'info>,
    /// CHECK: beliebiges Strategy‑Programm
    pub strategy_program: UncheckedAccount<'info>,
}

// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Diese Strategie ist bereits registriert.")]
    StrategyAlreadyAdded,
}
