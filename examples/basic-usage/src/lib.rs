use {
    lotic::{declare_program, instruction, Context, InstructionAccounts},
    pinocchio::{AccountView, ProgramResult, entrypoint},
};

declare_program!("qWi7Aia7isECbFw7r5mE54rqRb3GZBDMkaCsMDX6dox");

#[instruction]
fn initialize(ctx: &Context<Initialize>) -> ProgramResult {
    ctx.accounts
        .authority
        .set_lamports(ctx.accounts.authority.lamports().checked_sub(5).unwrap());
    ctx.accounts
        .data_account
        .set_lamports(ctx.accounts.data_account.lamports().checked_add(5).unwrap());
    Ok(())
}

#[instruction]
fn aupdate(_ctx: &Context<Initialize>) -> ProgramResult {
    Ok(())
}

#[instruction]
fn update(_ctx: &Context<Initialize>) -> ProgramResult {
    Ok(())
}

#[derive(InstructionAccounts)]
pub struct Initialize<'view> {
    #[lotic(mut, signer)]
    pub authority: &'view AccountView,
    #[lotic(mut)]
    pub data_account: &'view AccountView,
    #[lotic(program = token)]
    pub token_program: &'view AccountView,
    #[lotic(program = system)]
    pub system_account: &'view AccountView,
}
