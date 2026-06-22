const FORMAT_RESET: &str = "§r";
const FORMAT_OBFUSCATED: &str = "§k";
const FORMAT_BOLD: &str = "§l";
const FORMAT_STRIKETHROUGH: &str = "§m";
const FORMAT_UNDERLINED: &str = "§n";
const FORMAT_ITALIC: &str = "§o";
const FORMAT_PREFIX: char = '§';

impl super::Text {
    /// Converts the [`super::Text`] object to a plain string with the [legacy formatting (`§` and format codes)](https://wiki.vg/Chat#Old_system)
    ///
    /// Removes everything that can't be represented with a `§` and a modifier.
    /// Any colors not on the [the legacy color list](https://wiki.vg/Chat#Colors) will be replaced with their closest equivalent.
    pub fn to_legacy_lossy(&self) -> String {
        let mut state = State::default();
        state.visit_tree(&self.0);
        state.finish()
    }
}

#[derive(Default, Clone)]
struct Modifiers {
    obfuscated: Option<bool>,
    bold: Option<bool>,
    strikethrough: Option<bool>,
    underlined: Option<bool>,
    italic: Option<bool>,
    color: Option<super::Color>,
}

impl Modifiers {
    fn render(&self) -> String {
        let mut output = self.render_color();
        output.push_str(&self.render_flags());
        output
    }

    fn render_color(&self) -> String {
        let Some(color) = self.color else {
            return String::new();
        };
        let Some(code) = color_code(color) else {
            return String::new();
        };
        format!("{FORMAT_PREFIX}{code}")
    }

    fn render_flags(&self) -> String {
        [
            marker_for(self.obfuscated, FORMAT_OBFUSCATED),
            marker_for(self.bold, FORMAT_BOLD),
            marker_for(self.strikethrough, FORMAT_STRIKETHROUGH),
            marker_for(self.underlined, FORMAT_UNDERLINED),
            marker_for(self.italic, FORMAT_ITALIC),
        ]
        .join("")
    }

    fn merged_with(&self, other: &Self) -> Self {
        Self {
            obfuscated: other.obfuscated.or(self.obfuscated),
            bold: other.bold.or(self.bold),
            strikethrough: other.strikethrough.or(self.strikethrough),
            underlined: other.underlined.or(self.underlined),
            italic: other.italic.or(self.italic),
            color: other.color.or(self.color),
        }
    }
}

fn color_code(color: super::Color) -> Option<char> {
    match color {
        super::Color::Rgb(rgb) => Some(rgb.to_named_lossy().hex_digit()),
        super::Color::Named(normal) => Some(normal.hex_digit()),
        super::Color::Reset => None,
    }
}

fn marker_for(value: Option<bool>, marker: &'static str) -> &'static str {
    if value == Some(true) {
        marker
    } else {
        ""
    }
}

#[derive(Default)]
struct State {
    output: String,
    modifiers: Modifiers,
}

impl State {
    fn finish(self) -> String {
        self.output
    }

    fn visit_tree(&mut self, root: &super::TextInner) {
        let mut stack = vec![root];
        while let Some(current) = stack.pop() {
            self.visit_component(current);
            stack.extend(current.extra.iter().rev().map(|child| &*child.0));
        }
    }

    fn visit_component(&mut self, component: &super::TextInner) {
        let new_modifiers = modifiers_from(component);
        self.push_modifier_delta(component, &new_modifiers);
        self.modifiers = self.modifiers.merged_with(&new_modifiers);
        self.push_text_content(&component.content);
    }

    fn push_modifier_delta(&mut self, component: &super::TextInner, new_modifiers: &Modifiers) {
        if removes_any_modifier(component) {
            self.output.push_str(FORMAT_RESET);
            self.output
                .push_str(&self.modifiers.merged_with(new_modifiers).render());
            return;
        }
        self.output.push_str(&new_modifiers.render());
    }

    fn push_text_content(&mut self, content: &super::TextContent) {
        if let super::TextContent::Text { text } = content {
            self.output.push_str(text);
        }
    }
}

fn modifiers_from(component: &super::TextInner) -> Modifiers {
    Modifiers {
        obfuscated: component.obfuscated,
        bold: component.bold,
        strikethrough: component.strikethrough,
        underlined: component.underlined,
        italic: component.italic,
        color: component.color,
    }
}

fn removes_any_modifier(component: &super::TextInner) -> bool {
    [
        component.obfuscated,
        component.bold,
        component.strikethrough,
        component.underlined,
        component.italic,
    ]
    .contains(&Some(false))
        || component.color == Some(super::Color::Reset)
}
