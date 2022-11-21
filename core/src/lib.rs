extern crate syn;
#[macro_use]
extern crate quote;

mod describable;

use proc_macro::TokenStream;

#[proc_macro_derive(Describe)]
pub fn describe(input: TokenStream) -> TokenStream {
    describable::describe_impl(input)
}
