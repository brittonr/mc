#![doc = include_str!("../README.md")]

type Cow<'a, T> = std::borrow::Cow<'a, T>;

mod codec;
pub mod color;
mod construct;
mod ops;
mod render;
#[cfg(test)]
mod tests;
mod textlike;

pub use color::Color;
pub use textlike::IntoText;

/// Represents formatted text in Minecraft's JSON text format.
///
/// Text is used in various places such as chat, window titles,
/// disconnect messages, written books, signs, and more.
///
/// For more information, see the relevant [Minecraft Wiki article].
///
/// [Minecraft Wiki article]: https://minecraft.wiki/w/Raw_JSON_text_format
///
/// # Examples
///
/// With [`IntoText`] in scope, you can write the following:
/// ```
/// use valence_text::{Color, IntoText, Text};
///
/// let txt = "The text is ".into_text()
///     + "Red".color(Color::RED)
///     + ", "
///     + "Green".color(Color::GREEN)
///     + ", and also "
///     + "Blue".color(Color::BLUE)
///     + "! And maybe even "
///     + "Italic".italic()
///     + ".";
///
/// assert_eq!(
///     txt.to_string(),
///     r#"{"text":"The text is ","extra":[{"text":"Red","color":"red"},{"text":", "},{"text":"Green","color":"green"},{"text":", and also "},{"text":"Blue","color":"blue"},{"text":"! And maybe even "},{"text":"Italic","italic":true},{"text":"."}]}"#
/// );
/// ```
#[derive(Clone, PartialEq, Default, serde::Serialize)]
#[serde(transparent)]
pub struct Text(Box<TextInner>);

/// Text data and formatting.
#[derive(Clone, PartialEq, Default, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextInner {
    #[serde(flatten)]
    pub content: TextContent,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<Font>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insertion: Option<Cow<'static, str>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<Text>,
}

/// The text content of a Text object.
#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum TextContent {
    /// Normal text
    Text { text: Cow<'static, str> },
    /// A piece of text that will be translated on the client based on the
    /// client language. If no corresponding translation can be found, the
    /// identifier itself is used as the translated text.
    Translate {
        /// A translation identifier, corresponding to the identifiers found in
        /// loaded language files.
        translate: Cow<'static, str>,
        /// Optional list of text components to be inserted into slots in the
        /// translation text. Ignored if `translate` is not present.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        with: Vec<Text>,
    },
    /// Displays a score holder's current score in an objective.
    ScoreboardValue { score: ScoreboardValueContent },
    /// Displays the name of one or more entities found by a [`selector`].
    ///
    /// [`selector`]: https://minecraft.wiki/w/Target_selectors
    EntityNames {
        /// A string containing a [`selector`].
        ///
        /// [`selector`]: https://minecraft.wiki/w/Target_selectors
        selector: Cow<'static, str>,
        /// An optional custom separator used when the selector returns multiple
        /// entities. Defaults to the ", " text with gray color.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        separator: Option<Text>,
    },
    /// Displays the name of the button that is currently bound to a certain
    /// configurable control on the client.
    Keybind {
        /// A [`keybind identifier`], to be displayed as the name of the button
        /// that is currently bound to that action.
        ///
        /// [`keybind identifier`]: https://minecraft.wiki/w/Controls#Configurable_controls
        keybind: Cow<'static, str>,
    },
    /// Displays NBT values from block entities.
    BlockNbt {
        block: Cow<'static, str>,
        nbt: Cow<'static, str>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interpret: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        separator: Option<Text>,
    },
    /// Displays NBT values from entities.
    EntityNbt {
        entity: Cow<'static, str>,
        nbt: Cow<'static, str>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interpret: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        separator: Option<Text>,
    },
    /// Displays NBT values from command storage.
    StorageNbt {
        storage: valence_ident::Ident<Cow<'static, str>>,
        nbt: Cow<'static, str>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interpret: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        separator: Option<Text>,
    },
}

/// Scoreboard value.
#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct ScoreboardValueContent {
    /// The name of the score holder whose score should be displayed. This
    /// can be a [`selector`] or an explicit name.
    ///
    /// [`selector`]: https://minecraft.wiki/w/Target_selectors
    pub name: Cow<'static, str>,
    /// The internal name of the objective to display the player's score in.
    pub objective: Cow<'static, str>,
    /// If present, this value is displayed regardless of what the score
    /// would have been.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Cow<'static, str>>,
}

/// Action to take on click of the text.
#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "action", content = "value", rename_all = "snake_case")]
pub enum ClickEvent {
    /// Opens an URL
    OpenUrl(Cow<'static, str>),
    /// Only usable by internal servers for security reasons.
    OpenFile(Cow<'static, str>),
    /// Sends a chat command. Doesn't actually have to be a command, can be a
    /// normal chat message.
    RunCommand(Cow<'static, str>),
    /// Replaces the contents of the chat box with the text, not necessarily a
    /// command.
    SuggestCommand(Cow<'static, str>),
    /// Only usable within written books. Changes the page of the book. Indexing
    /// starts at 1.
    ChangePage(i32),
    /// Copies the given text to clipboard
    CopyToClipboard(Cow<'static, str>),
}

/// Action to take when mouse-hovering on the text.
#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "action", content = "contents", rename_all = "snake_case")]
// Clippy: Minecraft's wire names intentionally share the `show_` prefix.
#[allow(clippy::enum_variant_names)]
pub enum HoverEvent {
    /// Displays a tooltip with the given text.
    ShowText(Text),
    /// Shows an item.
    ShowItem {
        /// Resource identifier of the item
        id: valence_ident::Ident<Cow<'static, str>>,
        /// Number of the items in the stack
        count: Option<i32>,
        /// NBT information about the item (sNBT format)
        tag: Cow<'static, str>,
    },
    /// Shows an entity.
    ShowEntity {
        /// The entity's UUID
        id: uuid::Uuid,
        /// Resource identifier of the entity
        #[serde(rename = "type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        kind: Option<valence_ident::Ident<Cow<'static, str>>>,
        /// Optional custom name for the entity
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<Text>,
    },
}

/// The font of the text.
#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Font {
    /// The default font.
    #[serde(rename = "minecraft:default")]
    Default,
    /// Unicode font.
    #[serde(rename = "minecraft:uniform")]
    Uniform,
    /// Enchanting table font.
    #[serde(rename = "minecraft:alt")]
    Alt,
}
