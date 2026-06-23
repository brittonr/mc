#![doc = include_str!("../README.md")]

mod expand;
mod node;
mod path;

// Proc-macro API name is mandated by the public `Command` derive.
#[proc_macro_derive(Command, attributes(command, scopes, paths, suggestions))]
pub fn derive_command(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match expand::run(input) {
        Ok(expansion) => expansion,
        Err(err) => err.to_compile_error().into(),
    }
}
