pub use lotic_macros::{InstructionAccounts, declare_program, instruction};
use pinocchio::Address;

pub struct Context<'a, T> {
    pub program_id: &'a Address,
    pub accounts: &'a mut T,
}
