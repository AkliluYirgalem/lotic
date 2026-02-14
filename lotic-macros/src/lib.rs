use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{ItemMod, parse_macro_input},
};

#[proc_macro_attribute]
pub fn program(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemMod);
    let content = input.content.as_ref().unwrap();

    for item in content.1.iter() {
        if let syn::Item::Fn(func) = item {
            if let syn::Visibility::Public(_) = func.vis {
                println!("{}", func.sig.ident);
            }
        }
    }
    let expanded = quote! {
        #input
    };

    TokenStream::from(expanded)
}
