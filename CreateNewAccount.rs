

///Functions

pub fn creat(ctx: Contexts<newaccountcontext>, owner : pubkey) -> result<()>{
let creation: &mut newaccountdeposit = &mut ctx.accounts.newaccountdeposit;
creation.name = 0;
creation.owner = owner;
creation.userdeposit = Vec::new();

}

////Contexts

pub struct newaccountcontext <'info> {

    #[account(init, payer = user, space = 8)]
    pub newaccount: Accounts<'info, newaccountdeposit>,
    pub user: signer<'info>,
    pub system_program: system_program<'info, system>

}

////Accounts

#[account]

pub struct newaccountdeposit {

    pub total_deposit : u64,
    pub userdeposit : Vec<userdeposit>,
    pub owner : pubkey
}


