// Clippy: `Text::text` is the public constructor matching the wire component
// kind.
#[allow(clippy::self_named_constructors)]
impl super::Text {
    /// Constructs a new plain text object.
    pub fn text<P>(plain: P) -> Self
    where
        P: Into<super::Cow<'static, str>>,
    {
        Self(Box::new(super::TextInner {
            content: super::TextContent::Text { text: plain.into() },
            ..Default::default()
        }))
    }

    /// Create translated text based on the given translation key, with extra
    /// text components to be inserted into the slots of the translation text.
    pub fn translate<K, W>(key: K, with: W) -> Self
    where
        K: Into<super::Cow<'static, str>>,
        W: Into<Vec<super::Text>>,
    {
        Self(Box::new(super::TextInner {
            content: super::TextContent::Translate {
                translate: key.into(),
                with: with.into(),
            },
            ..Default::default()
        }))
    }

    /// Create a score from the scoreboard with an optional custom value.
    pub fn score<N, O>(name: N, objective: O, value: Option<super::Cow<'static, str>>) -> Self
    where
        N: Into<super::Cow<'static, str>>,
        O: Into<super::Cow<'static, str>>,
    {
        Self(Box::new(super::TextInner {
            content: super::TextContent::ScoreboardValue {
                score: super::ScoreboardValueContent {
                    name: name.into(),
                    objective: objective.into(),
                    value,
                },
            },
            ..Default::default()
        }))
    }

    /// Creates a text component for selecting entity names with an optional
    /// custom separator.
    pub fn selector<S>(selector: S, separator: Option<super::Text>) -> Self
    where
        S: Into<super::Cow<'static, str>>,
    {
        Self(Box::new(super::TextInner {
            content: super::TextContent::EntityNames {
                selector: selector.into(),
                separator,
            },
            ..Default::default()
        }))
    }

    /// Creates a text component for a keybind. The keybind should be a valid
    /// [`keybind identifier`].
    ///
    /// [`keybind identifier`]: https://minecraft.wiki/w/Controls#Configurable_controls
    pub fn keybind<K>(keybind: K) -> Self
    where
        K: Into<super::Cow<'static, str>>,
    {
        Self(Box::new(super::TextInner {
            content: super::TextContent::Keybind {
                keybind: keybind.into(),
            },
            ..Default::default()
        }))
    }

    /// Creates a text component for a block NBT tag.
    pub fn block_nbt<B, N>(
        block: B,
        nbt: N,
        interpret: Option<bool>,
        separator: Option<super::Text>,
    ) -> Self
    where
        B: Into<super::Cow<'static, str>>,
        N: Into<super::Cow<'static, str>>,
    {
        Self(Box::new(super::TextInner {
            content: super::TextContent::BlockNbt {
                block: block.into(),
                nbt: nbt.into(),
                interpret,
                separator,
            },
            ..Default::default()
        }))
    }

    /// Creates a text component for an entity NBT tag.
    pub fn entity_nbt<E, N>(
        entity: E,
        nbt: N,
        interpret: Option<bool>,
        separator: Option<super::Text>,
    ) -> Self
    where
        E: Into<super::Cow<'static, str>>,
        N: Into<super::Cow<'static, str>>,
    {
        Self(Box::new(super::TextInner {
            content: super::TextContent::EntityNbt {
                entity: entity.into(),
                nbt: nbt.into(),
                interpret,
                separator,
            },
            ..Default::default()
        }))
    }

    /// Creates a text component for a command storage NBT tag.
    pub fn storage_nbt<S, N>(
        storage: S,
        nbt: N,
        interpret: Option<bool>,
        separator: Option<super::Text>,
    ) -> Self
    where
        S: Into<valence_ident::Ident<super::Cow<'static, str>>>,
        N: Into<super::Cow<'static, str>>,
    {
        Self(Box::new(super::TextInner {
            content: super::TextContent::StorageNbt {
                storage: storage.into(),
                nbt: nbt.into(),
                interpret,
                separator,
            },
            ..Default::default()
        }))
    }

    /// Returns `true` if the text contains no characters. Returns `false`
    /// otherwise.
    pub fn is_empty(&self) -> bool {
        self.0.extra.iter().all(super::Text::is_empty) && content_is_empty(&self.0.content)
    }
}

fn content_is_empty(content: &super::TextContent) -> bool {
    match content {
        super::TextContent::Text { text } => text.is_empty(),
        super::TextContent::Translate { translate, .. } => translate.is_empty(),
        super::TextContent::ScoreboardValue { score } => {
            score.name.is_empty() || score.objective.is_empty()
        }
        super::TextContent::EntityNames { selector, .. } => selector.is_empty(),
        super::TextContent::Keybind { keybind } => keybind.is_empty(),
        super::TextContent::BlockNbt { nbt, .. }
        | super::TextContent::EntityNbt { nbt, .. }
        | super::TextContent::StorageNbt { nbt, .. } => nbt.is_empty(),
    }
}
