use heck::ToPascalCase;

type TokenStream = proc_macro2::TokenStream;

#[derive(serde::Deserialize, Debug)]
struct Sound {
    id: u16,
    name: String,
}

pub(crate) fn build() -> anyhow::Result<TokenStream> {
    valence_build_utils::rerun_if_changed(["extracted/sounds.json"]);

    let sounds = serde_json::from_str::<Vec<Sound>>(include_str!("../extracted/sounds.json"))?;
    debug_assert!(!sounds.is_empty(), "generated sound list is non-empty");
    Ok(tokens(&sounds))
}

fn tokens(sounds: &[Sound]) -> TokenStream {
    debug_assert!(!sounds.is_empty(), "generated sound list is non-empty");
    let sound_count = sounds.len();
    let sound_from_raw_id_arms = raw_id_from_arms(sounds);
    let sound_to_raw_id_arms = raw_id_to_arms(sounds);
    let sound_from_ident_arms = ident_from_arms(sounds);
    let sound_to_ident_arms = ident_to_arms(sounds);
    let sound_variants = variants(sounds);

    quote::quote! {
        #[doc = "Represents a sound from the game"]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum Sound {
            #(#sound_variants,)*
        }

        impl Sound {
            #[doc = "Constructs a sound from a raw item ID."]
            #[doc = ""]
            #[doc = "If the given ID is invalid, `None` is returned."]
            pub const fn from_raw(id: u16) -> Option<Self> {
                match id {
                    #sound_from_raw_id_arms
                    _ => None
                }
            }

            #[doc = "Gets the raw sound ID from the sound"]
            pub const fn to_raw(self) -> u16 {
                match self {
                    #sound_to_raw_id_arms
                }
            }

            #[doc = "Construct a sound from its `snake_case` name."]
            #[doc = ""]
            #[doc = "Returns `None` if the name is invalid."]
            pub fn from_ident(id: valence_ident::Ident<&str>) -> Option<Self> {
                match id.as_str() {
                    #sound_from_ident_arms
                    _ => None
                }
            }

            #[doc = "Gets the identifier of this sound."]
            pub const fn to_ident(self) -> valence_ident::Ident<&'static str> {
                match self {
                    #sound_to_ident_arms
                }
            }

            #[doc = "An array of all sounds."]
            pub const ALL: [Self; #sound_count] = [#(Self::#sound_variants,)*];
        }
    }
}

fn raw_id_from_arms(sounds: &[Sound]) -> TokenStream {
    sounds
        .iter()
        .map(|sound| {
            let id = &sound.id;
            let name = variant_name(sound);

            quote::quote! {
                #id => Some(Self::#name),
            }
        })
        .collect()
}

fn raw_id_to_arms(sounds: &[Sound]) -> TokenStream {
    sounds
        .iter()
        .map(|sound| {
            let id = &sound.id;
            let name = variant_name(sound);

            quote::quote! {
                Self::#name => #id,
            }
        })
        .collect()
}

fn ident_from_arms(sounds: &[Sound]) -> TokenStream {
    sounds
        .iter()
        .map(|sound| {
            let ident_name = format!("minecraft:{}", &sound.name);
            let name = variant_name(sound);
            quote::quote! {
                #ident_name => Some(Self::#name),
            }
        })
        .collect()
}

fn ident_to_arms(sounds: &[Sound]) -> TokenStream {
    sounds
        .iter()
        .map(|sound| {
            let str_name = &sound.name;
            let name = variant_name(sound);
            quote::quote! {
                Self::#name => valence_ident::ident!(#str_name),
            }
        })
        .collect()
}

fn variants(sounds: &[Sound]) -> Vec<proc_macro2::Ident> {
    sounds.iter().map(variant_name).collect()
}

fn variant_name(sound: &Sound) -> proc_macro2::Ident {
    valence_build_utils::ident(sound.name.to_pascal_case())
}
