#[macro_use]
extern crate quote;
extern crate proc_macro;

use quote::format_ident;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GrammarParser, attributes(name))]
pub fn derive_grammar_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident.to_string();
    let name = struct_name.replace("Parser", "").to_lowercase();

    let struct_name = format_ident!("{}", struct_name);
    let format_fn = format_ident!("format_{}", name);
    let lint_fn = format_ident!("lint_{}", name);

    quote! {
        #[allow(dead_code)]
        pub fn #format_fn(text: &str) -> FormatResult {{
            let pairs = #struct_name::parse(Rule::item, text);
            let text = code::FormatResult::new(text);
            code::format_pairs(text, pairs)
        }}

        #[allow(dead_code)]
        pub fn #lint_fn(text: &str) -> LintResult {{
            let pairs = #struct_name::parse(Rule::item, text);
            let text = code::LintResult::new(text);
            code::format_pairs(text, pairs)
        }}
    }
    .into()
}
