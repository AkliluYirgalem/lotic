pub use lotic_macros::{declare_program, instruction, InstructionAccounts};
use pinocchio::Address;

pub struct Context<'a, T> {
    pub program_id: &'a Address,
    pub accounts: &'a mut T,
}
