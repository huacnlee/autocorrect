extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GrammarParser, attributes(name))]
pub fn derive_grammar_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident.to_string();
    let name = struct_name.replace("Parser", "").to_lowercase();

    let code = format!(
        r##"
        #[allow(dead_code)]
        pub fn format_{name}(text: &str) -> FormatResult {{
            let pairs = {struct_name}::parse(Rule::item, text);
            let text = code::FormatResult::new(text);
            code::format_pairs(text, pairs)
        }}
        
        #[allow(dead_code)]
        pub fn lint_{name}(text: &str) -> LintResult {{
            let pairs =  {struct_name}::parse(Rule::item, text);
            let text = code::LintResult::new(text);
            code::format_pairs(text, pairs)
        }}
        "##,
        name = name.as_str(),
        struct_name = struct_name.as_str()
    );

    code.parse().unwrap()
}
