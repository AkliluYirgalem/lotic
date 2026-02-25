use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, Data, DeriveInput, Fields},
};

pub fn instruction_accounts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = input.ident;

    let mut validations = Vec::new();
    let mut field_idents = Vec::new();

    let Data::Struct(data) = input.data else {
        return TokenStream::new();
    };

    let Fields::Named(fields) = data.fields else {
        return TokenStream::new();
    };

    for field in fields.named {
        let field_ident = field.ident.unwrap(); // Because We are expecting named structs.
        field_idents.push(field_ident.clone());

        for attr in &field.attrs {
            if attr.path().is_ident("lotic") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("signer") {
                        validations.push(quote! {
                            if !self.#field_ident.is_signer() {
                                return Err(ProgramError::MissingRequiredSignature);
                            }
                        });
                    }
                    Ok(())
                });
            }
        }
    }

    let expanded = quote! {
        impl <'view> core::convert::TryFrom<&'view [AccountView]> for #struct_ident <'view>  {
            type Error = ::pinocchio::error::ProgramError;

            fn try_from(accounts: &'view [AccountView]) -> Result<Self, Self::Error> {
                let [#(#field_idents,)* ..] = accounts else {
                    return Err(::pinocchio::error::ProgramError::NotEnoughAccountKeys);
                };

                let accounts = Self {
                    #(#field_idents,)*
                };

                // accounts.check_constraints()?;
                Ok(accounts)
            }
        }
    };

    expanded.into()
}
