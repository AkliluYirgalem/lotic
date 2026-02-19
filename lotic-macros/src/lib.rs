use proc_macro::TokenStream;

mod instruction;
mod instruction_accounts;

#[proc_macro_attribute]
pub fn instruction(attr: TokenStream, item: TokenStream) -> TokenStream {
    instruction::instruction(attr, item)
}

#[proc_macro_derive(InstructionAccounts, attributes(lotic))]
pub fn instruction_accounts(input: TokenStream) -> TokenStream {
    instruction_accounts::instruction_accounts(input)
}
