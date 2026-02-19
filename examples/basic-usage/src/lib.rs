use {
    lotic::{Context, InstructionAccounts, instruction},
    pinocchio::{AccountView, ProgramResult},
};

#[instruction]
fn initialize(ctx: &Context<Initialize>, name: &u64, age: &u64) -> ProgramResult {
    Ok(())
}

#[instruction]
fn aupdate(ctx: &Context<Initialize>, name: &str, age: &u64) -> ProgramResult {
    Ok(())
}

#[instruction]
fn update(ctx: &Context<Initialize>, name: &str, age: &u64) -> ProgramResult {
    Ok(())
}

// pinocchio::entrypoint!(__process_instruction__);

// pub fn __process_instruction__(
//     _program_id: &pinocchio::Address,
//     _accounts: &[AccountView],
//     instruction_data: &[u8],
// ) -> ProgramResult {
//     let (ix, _args) = instruction_data
//         .split_first()
//         .ok_or(pinocchio::error::ProgramError::InvalidInstructionData)?;

//     match *ix {
//         0 => {
//             // todo

//             //hello_lotic::_initialize(ctx)
//             Ok(())
//         }
//         _ =>
// ProgramResult::Err(pinocchio::error::ProgramError::InvalidInstructionData),
//     }
// }

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
