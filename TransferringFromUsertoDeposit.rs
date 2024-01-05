pub fn transferringDeposit (ctx: Context<transferringDepositContext, amount : u64){

    anchor_lang::solana_program::program::invoke (

        &system_instruction::transfer(

            &ctx.account.user.key(),
            &ctx.account.total_deposit.key()
            amount,
         ),
            &[
                 ctx.accounts.user.to_account_info(),
                 ctx.accounts.deposit_account.to_account_info(),
             ],
    )?;

}
