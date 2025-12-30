#![feature(associated_type_defaults)]

mod parse;
mod generate;
#[proc_macro]
pub fn virtualize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate::generate(input)
}
