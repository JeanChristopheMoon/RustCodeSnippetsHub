use anchor_lang::prelude::*;

declare_id!("3qHNM98iLTaQtwmj2NkViXnHZQjNBS5PTHT2AuPxHXYN");

#[program]
pub mod nft_minter {
    use super::*;

    pub fn mint_nft(ctx: Context<CreateToken>, nft_name: String, nft_symbol: String, nft_uri: String) -> Result<()> {
        mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint_account.to_account_info(),
                    to: ctx.accounts.associated_token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            1,
        )?;
