//! Provides the [`IntoText`] trait and implementations.

mod convert;

#[cfg(test)]
mod tests;

/// Trait for any data that can be converted to a [`super::Text`] object.
///
/// Also conveniently provides many useful methods for modifying a
/// [`super::Text`] object.
///
/// # Usage
///
/// ```
/// # use valence_text::{IntoText, color::NamedColor};
/// let mut my_text = "".into_text();
/// my_text = my_text.color(NamedColor::Red).bold();
/// my_text = my_text.add_child("CRABBBBB".obfuscated());
/// ```
pub trait IntoText<'a>: Sized {
    /// Converts to a [`super::Text`] object, either owned or borrowed.
    fn into_cow_text(self) -> super::Cow<'a, super::Text>;

    /// Converts to an owned [`super::Text`] object.
    fn into_text(self) -> super::Text {
        self.into_cow_text().into_owned()
    }

    /// Sets the color of the text.
    fn color(self, color: impl Into<super::Color>) -> super::Text {
        let mut value = self.into_text();
        value.color = Some(color.into());
        value
    }
    /// Clears the color of the text. Color of parent [`super::Text`] object
    /// will be used.
    fn clear_color(self) -> super::Text {
        let mut value = self.into_text();
        value.color = None;
        value
    }

    /// Sets the font of the text.
    fn font(self, font: super::Font) -> super::Text {
        let mut value = self.into_text();
        value.font = Some(font);
        value
    }
    /// Clears the font of the text. Font of parent [`super::Text`] object will
    /// be used.
    fn clear_font(self) -> super::Text {
        let mut value = self.into_text();
        value.font = None;
        value
    }

    /// Makes the text bold.
    fn bold(self) -> super::Text {
        let mut value = self.into_text();
        value.bold = Some(true);
        value
    }
    /// Makes the text not bold.
    fn not_bold(self) -> super::Text {
        let mut value = self.into_text();
        value.bold = Some(false);
        value
    }
    /// Clears the `bold` property of the text. Property of the parent
    /// [`super::Text`] object will be used.
    fn clear_bold(self) -> super::Text {
        let mut value = self.into_text();
        value.bold = None;
        value
    }

    /// Makes the text italic.
    fn italic(self) -> super::Text {
        let mut value = self.into_text();
        value.italic = Some(true);
        value
    }
    /// Makes the text not italic.
    fn not_italic(self) -> super::Text {
        let mut value = self.into_text();
        value.italic = Some(false);
        value
    }
    /// Clears the `italic` property of the text. Property of the parent
    /// [`super::Text`] object will be used.
    fn clear_italic(self) -> super::Text {
        let mut value = self.into_text();
        value.italic = None;
        value
    }

    /// Makes the text underlined.
    fn underlined(self) -> super::Text {
        let mut value = self.into_text();
        value.underlined = Some(true);
        value
    }
    /// Makes the text not underlined.
    fn not_underlined(self) -> super::Text {
        let mut value = self.into_text();
        value.underlined = Some(false);
        value
    }
    /// Clears the `underlined` property of the text. Property of the parent
    /// [`super::Text`] object will be used.
    fn clear_underlined(self) -> super::Text {
        let mut value = self.into_text();
        value.underlined = None;
        value
    }

    /// Adds a strikethrough effect to the text.
    fn strikethrough(self) -> super::Text {
        let mut value = self.into_text();
        value.strikethrough = Some(true);
        value
    }
    /// Removes the strikethrough effect from the text.
    fn not_strikethrough(self) -> super::Text {
        let mut value = self.into_text();
        value.strikethrough = Some(false);
        value
    }
    /// Clears the `strikethrough` property of the text. Property of the parent
    /// [`super::Text`] object will be used.
    fn clear_strikethrough(self) -> super::Text {
        let mut value = self.into_text();
        value.strikethrough = None;
        value
    }

    /// Makes the text obfuscated.
    fn obfuscated(self) -> super::Text {
        let mut value = self.into_text();
        value.obfuscated = Some(true);
        value
    }
    /// Makes the text not obfuscated.
    fn not_obfuscated(self) -> super::Text {
        let mut value = self.into_text();
        value.obfuscated = Some(false);
        value
    }
    /// Clears the `obfuscated` property of the text. Property of the parent
    /// [`super::Text`] object will be used.
    fn clear_obfuscated(self) -> super::Text {
        let mut value = self.into_text();
        value.obfuscated = None;
        value
    }

    /// Adds an `insertion` property to the text. When shift-clicked, the given
    /// text will be inserted into chat box for the client.
    fn insertion(self, insertion: impl Into<super::Cow<'static, str>>) -> super::Text {
        let mut value = self.into_text();
        value.insertion = Some(insertion.into());
        value
    }
    /// Clears the `insertion` property of the text. Property of the parent
    /// [`super::Text`] object will be used.
    fn clear_insertion(self) -> super::Text {
        let mut value = self.into_text();
        value.insertion = None;
        value
    }

    /// On click, opens the given URL. Has to be `http` or `https` protocol.
    fn on_click_open_url(self, url: impl Into<super::Cow<'static, str>>) -> super::Text {
        let mut value = self.into_text();
        value.click_event = Some(super::ClickEvent::OpenUrl(url.into()));
        value
    }
    /// On click, sends a command. Doesn't actually have to be a command, can be
    /// a simple chat message.
    fn on_click_run_command(self, command: impl Into<super::Cow<'static, str>>) -> super::Text {
        let mut value = self.into_text();
        value.click_event = Some(super::ClickEvent::RunCommand(command.into()));
        value
    }
    /// On click, copies the given text to the chat box.
    fn on_click_suggest_command(self, command: impl Into<super::Cow<'static, str>>) -> super::Text {
        let mut value = self.into_text();
        value.click_event = Some(super::ClickEvent::SuggestCommand(command.into()));
        value
    }
    /// On click, turns the page of the opened book to the given number.
    /// Indexing starts at `1`.
    fn on_click_change_page(self, page: impl Into<i32>) -> super::Text {
        let mut value = self.into_text();
        value.click_event = Some(super::ClickEvent::ChangePage(page.into()));
        value
    }
    /// On click, copies the given text to clipboard.
    fn on_click_copy_to_clipboard(self, text: impl Into<super::Cow<'static, str>>) -> super::Text {
        let mut value = self.into_text();
        value.click_event = Some(super::ClickEvent::CopyToClipboard(text.into()));
        value
    }
    /// Clears the `click_event` property of the text. Property of the parent
    /// [`super::Text`] object will be used.
    fn clear_click_event(self) -> super::Text {
        let mut value = self.into_text();
        value.click_event = None;
        value
    }

    /// On mouse hover, shows the given text in a tooltip.
    fn on_hover_show_text(self, text: impl IntoText<'static>) -> super::Text {
        let mut value = self.into_text();
        value.hover_event = Some(super::HoverEvent::ShowText(text.into_text()));
        value
    }
    /// Clears the `hover_event` property of the text. Property of the parent
    /// [`super::Text`] object will be used.
    fn clear_hover_event(self) -> super::Text {
        let mut value = self.into_text();
        value.hover_event = None;
        value
    }

    /// Adds a child [`super::Text`] object.
    fn add_child(self, text: impl IntoText<'static>) -> super::Text {
        let mut value = self.into_text();
        value.extra.push(text.into_text());
        value
    }
}
