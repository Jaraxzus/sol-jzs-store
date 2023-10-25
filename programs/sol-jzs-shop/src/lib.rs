use anchor_lang::prelude::*;
// use anchor_spl::token::{Mint, TokenAccount, Transfer};
// use anchor_spl::token;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use anchor_lang::system_program::transfer;

declare_id!("GVu78SDpdrVAWhTa11oFQg6oiVDbL74c7UyTx8XKjEWb");

#[program]
pub mod sol_jzs_shop {

    use super::*;

    pub fn initialize_store(
        ctx: Context<InitializeStore>,
        // token_mint: Pubkey,
        initial_price: u64,
    ) -> Result<String> {
        // Создать аккаунт магазина
        // let pk = Pubkey::default();
        // msg!("{}", pk);
        // ctx.accounts.store.store_account = pk;
        // if token_mint.()
        // ctx.accounts.store.token_mint = token_mint;
        ctx.accounts.store.price = initial_price;

        // Создать аккаунт для хранения цены
        // let price_account = Pubkey::new_unique();
        // ctx.accounts.price = &mut price_account;

        // Установить начальную цену
        // **ctx.accounts.price = 100;

        Ok("Store initialized".to_string())
    }
    pub fn update_price(
        ctx: Context<UpdatePrice>,
        price: u64,
    ) -> Result<u64> {
       ctx.accounts.store.price = price;
       Ok(ctx.accounts.store.price)
    }
    pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
        // Create the Transfer struct for our context
        let transfer_instruction = Transfer { 
            from: ctx.accounts.store.to_account_info(), 
            to: ctx.accounts.user.to_account_info(), 
            authority: ctx.accounts.user.to_account_info() };
         
        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the Context for our Transfer request
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        // Execute anchor's helper function to transfer tokens
        anchor_spl::token::transfer(cpi_ctx, amount)?;
 
        Ok(())
    }
            // pub fn sell() {}
}

#[derive(Accounts)]
#[instruction(coin_key: Pubkey)]
pub struct InitializeStore<'info> {
    #[account(
        init,
        payer = user,
        seeds = [&coin_key.to_bytes(), user.key().as_ref()],
        bump, 
        space = 8 + 8
    )]
    pub store: Account<'info, Store>,

    // pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut)]
    pub store: Account<'info, Store>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Buy<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: The associated token account that we are transferring the token from
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    #[account(mut)]
    pub store: Account<'info, Store>,
    /// CHECK: The associated token account that we are transferring the token to
    #[account(mut)]
    pub to: AccountInfo<'info>,
    // the authority of the from account 
    pub from_authority: Signer<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Store {
    // pub token_mint: Pubkey,
    pub price: u64,
}

// #[derive(Accounts)]
// pub struct TransferToken<'info> {
//     pub token_program: Program<'info, Token>,
//     /// CHECK: The associated token account that we are transferring the token from
//     #[account(mut)]
//     pub from: UncheckedAccount<'info>,
//     /// CHECK: The associated token account that we are transferring the token to
//     #[account(mut)]
//     pub to: AccountInfo<'info>,
//     // the authority of the from account 
//     pub from_authority: Signer<'info>,
// }
