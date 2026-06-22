use heck::ToShoutySnakeCase;

const TRANSLATION_KEYS_PATH: &str = "extracted/translation_keys.json";

type TokenStream = proc_macro2::TokenStream;

pub fn main() -> anyhow::Result<()> {
    valence_build_utils::rerun_if_changed([TRANSLATION_KEYS_PATH]);
    valence_build_utils::write_generated_file(build(TRANSLATION_KEYS_PATH)?, "translation_keys.rs")
}

fn build(path: impl AsRef<std::path::Path>) -> anyhow::Result<TokenStream> {
    let raw_translations = std::fs::read_to_string(path)?;
    let translations = parse_translations(&raw_translations)?;
    Ok(render_translation_keys(&translations))
}

fn parse_translations(raw_translations: &str) -> anyhow::Result<Vec<Translation>> {
    Ok(serde_json::from_str::<Vec<Translation>>(raw_translations)?)
}

fn render_translation_keys(translations: &[Translation]) -> TokenStream {
    let translation_key_consts = translations.iter().map(render_translation_key_const);

    quote::quote! {
        #(#translation_key_consts)*
    }
}

fn render_translation_key_const(translation: &Translation) -> TokenStream {
    let const_id = valence_build_utils::ident(translation.key.to_shouty_snake_case());
    let key = &translation.key;
    let doc = translation_doc(&translation.english_translation);

    quote::quote! {
        #[doc = #doc]
        pub const #const_id: &str = #key;
    }
}

fn translation_doc(english_translation: &str) -> String {
    format!("\"{}\"", escape_doc_text(english_translation)).replace('`', "\\`")
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Translation {
    key: String,
    english_translation: String,
}

/// Escapes characters that have special meaning inside docs.
fn escape_doc_text(text: &str) -> String {
    text.replace('[', "\\[").replace(']', "\\]")
}
