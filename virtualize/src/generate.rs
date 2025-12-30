use proc_macro::TokenStream;
use syn::parse_macro_input;
use crate::parse::Context;

pub fn generate(input: TokenStream) -> TokenStream {
    let _context = parse_macro_input!(input as Context);

    todo!()
}