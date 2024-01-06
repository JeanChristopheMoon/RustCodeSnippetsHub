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

///user_key is a variable that holds the public key
/// of the user who is currently attempting to perform
// an operation, such as a withdrawal.

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let user_key = ctx.accounts.user.key();
    let mut deposit_found = false;

    for deposit in ctx.accounts.deposit_account.user_deposits.iter_mut() {
        if deposit.user == user_key {
            require!(deposit.amount >= amount, ErrorCode::InsufficientFunds);
            deposit_found = true;

            deposit.amount -= amount;
            ctx.accounts.deposit_account.total_deposits -= amount;
            break;
        }
    }