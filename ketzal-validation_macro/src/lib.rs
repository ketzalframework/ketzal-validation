mod basic;
mod schema;
mod utils;

use proc_macro::TokenStream;

#[proc_macro_derive(KetzalBasicM, attributes(rule))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    basic::derive_validate(input)
}

#[proc_macro_derive(KetzalSchema, attributes(schema))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    schema::derive_schema(input)
}
