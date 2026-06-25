//! Pure JSON parsing and rendering helpers for runner configuration and evidence.
//!
//! The runner intentionally avoids a JSON dependency in this crate. This module
//! owns the small in-memory parser/renderer surface used by config readers,
//! receipt validators, and evidence renderers. It does not read files, write
//! files, inspect environment variables, or print diagnostics.

const JSON_STRING_QUOTE_COUNT: usize = 2;
const JSON_UNICODE_ESCAPE_HEX_DIGITS: usize = 4;
const JSON_HEX_RADIX: u32 = 16;

pub(crate) fn json_object_string_field(
    text: &str,
    object: &str,
    key: &str,
) -> Result<String, String> {
    json_string_field(json_object_slice(text, object)?, key)
}

pub(crate) fn json_object_optional_string_field(
    text: &str,
    object: &str,
    key: &str,
) -> Result<Option<String>, String> {
    json_optional_string_field(json_object_slice(text, object)?, key)
}

pub(crate) fn json_object_u32_field(text: &str, object: &str, key: &str) -> Result<u32, String> {
    json_u32_field(json_object_slice(text, object)?, key)
}

pub(crate) fn json_object_bool_field(text: &str, object: &str, key: &str) -> Result<bool, String> {
    json_bool_field(json_object_slice(text, object)?, key)
}

pub(crate) fn json_object_slice<'a>(text: &'a str, object: &str) -> Result<&'a str, String> {
    let key = format!("\"{object}\"");
    let mut search_start = 0usize;
    while let Some(relative_start) = text[search_start..].find(&key) {
        let start = search_start + relative_start;
        let after_key = &text[start + key.len()..];
        let after_colon = match after_key.trim_start().strip_prefix(':') {
            Some(value) => value,
            None => {
                search_start = start + key.len();
                continue;
            }
        };
        let brace_offset = after_colon
            .find('{')
            .ok_or_else(|| format!("missing object body for {object}"))?;
        let body_start = text.len() - after_colon.len() + brace_offset;
        let mut depth = 0usize;
        for (offset, ch) in text[body_start..].char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(&text[body_start..=body_start + offset]);
                    }
                }
                _ => {}
            }
        }
        return Err(format!("unterminated object {object}"));
    }
    Err(format!("missing object {object}"))
}

pub(crate) fn json_string_field(text: &str, key: &str) -> Result<String, String> {
    let after_colon = json_field_value(text, key)?;
    parse_json_string(after_colon).map(|(value, _)| value)
}

pub(crate) fn json_optional_string_field(text: &str, key: &str) -> Result<Option<String>, String> {
    let Some(after_colon) = json_field_value_opt(text, key)? else {
        return Ok(None);
    };
    if after_colon.trim_start().starts_with("null") {
        Ok(None)
    } else {
        parse_json_string(after_colon).map(|(value, _)| Some(value))
    }
}

pub(crate) fn json_optional_bool_field(text: &str, key: &str) -> Result<Option<bool>, String> {
    let Some(after_colon) = json_field_value_opt(text, key)? else {
        return Ok(None);
    };
    let after_colon = after_colon.trim_start();
    if after_colon.starts_with("true") {
        Ok(Some(true))
    } else if after_colon.starts_with("false") {
        Ok(Some(false))
    } else {
        Err(format!("field {key} must be a boolean"))
    }
}

pub(crate) fn json_optional_u32_field(text: &str, key: &str) -> Result<Option<u32>, String> {
    let Some(value) = json_field_value_opt(text, key)? else {
        return Ok(None);
    };
    parse_json_u32_value(key, value).map(Some)
}

pub(crate) fn json_u32_field(text: &str, key: &str) -> Result<u32, String> {
    parse_json_u32_value(key, json_field_value(text, key)?)
}

pub(crate) fn json_u64_field(text: &str, key: &str) -> Result<u64, String> {
    parse_json_u64_value(key, json_field_value(text, key)?)
}

fn parse_json_u32_value(key: &str, value: &str) -> Result<u32, String> {
    let value = value.trim_start();
    let digits: String = value.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        return Err(format!("field {key} is not an unsigned integer"));
    }
    digits
        .parse()
        .map_err(|e| format!("parse field {key}: {e}"))
}

fn parse_json_u64_value(key: &str, value: &str) -> Result<u64, String> {
    let value = value.trim_start();
    let digits: String = value.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        return Err(format!("field {key} is not an unsigned integer"));
    }
    digits
        .parse()
        .map_err(|e| format!("parse field {key}: {e}"))
}

pub(crate) fn json_optional_string_array_field(
    text: &str,
    key: &str,
) -> Result<Option<Vec<String>>, String> {
    let Some(value) = json_field_value_opt(text, key)? else {
        return Ok(None);
    };
    parse_json_string_array(value).map(Some)
}

pub(crate) fn json_bool_field(text: &str, key: &str) -> Result<bool, String> {
    let value = json_field_value(text, key)?.trim_start();
    if value.starts_with("true") {
        Ok(true)
    } else if value.starts_with("false") {
        Ok(false)
    } else {
        Err(format!("field {key} is not a bool"))
    }
}

pub(crate) fn json_field_value<'a>(text: &'a str, key: &str) -> Result<&'a str, String> {
    json_field_value_opt(text, key)?.ok_or_else(|| format!("missing field {key}"))
}

pub(crate) fn json_field_value_opt<'a>(
    text: &'a str,
    key: &str,
) -> Result<Option<&'a str>, String> {
    let needle = format!("\"{key}\"");
    let Some(start) = text.find(&needle) else {
        return Ok(None);
    };
    let after_key = &text[start + needle.len()..];
    let colon = after_key
        .find(':')
        .ok_or_else(|| format!("missing colon for field {key}"))?;
    Ok(Some(&after_key[colon + 1..]))
}

pub(crate) fn ensure_unique_json_field(text: &str, key: &str, context: &str) -> Result<(), String> {
    let count = json_field_occurrence_count(text, key);
    if count > 1 {
        return Err(format!("duplicate field {key} in {context}"));
    }
    Ok(())
}

fn json_field_occurrence_count(text: &str, key: &str) -> usize {
    let needle = format!("\"{key}\"");
    let mut count = 0usize;
    let mut search_start = 0usize;
    while let Some(relative_start) = text[search_start..].find(&needle) {
        let start = search_start + relative_start;
        let after_key = &text[start + needle.len()..];
        if after_key.trim_start().starts_with(':') {
            count += 1;
        }
        search_start = start + needle.len();
    }
    count
}

pub(crate) fn parse_json_string(text: &str) -> Result<(String, &str), String> {
    let text = text.trim_start();
    let mut chars = text.char_indices();
    match chars.next() {
        Some((_, '"')) => {}
        _ => return Err("expected JSON string".to_string()),
    }
    let mut out = String::new();
    while let Some((idx, ch)) = chars.next() {
        match ch {
            '"' => return Ok((out, &text[idx + ch.len_utf8()..])),
            '\\' => parse_json_escape(text, &mut chars, &mut out)?,
            other => out.push(other),
        }
    }
    Err("unterminated JSON string".to_string())
}

fn parse_json_escape(
    text: &str,
    chars: &mut std::str::CharIndices<'_>,
    out: &mut String,
) -> Result<(), String> {
    let Some((_, escaped)) = chars.next() else {
        return Err("unterminated JSON string escape".to_string());
    };
    match escaped {
        '"' => out.push('"'),
        '\\' => out.push('\\'),
        '/' => out.push('/'),
        'n' => out.push('\n'),
        'r' => out.push('\r'),
        't' => out.push('\t'),
        'u' => parse_json_unicode_escape(text, chars, out)?,
        other => return Err(format!("unsupported JSON escape \\{other}")),
    }
    Ok(())
}

fn parse_json_unicode_escape(
    text: &str,
    chars: &mut std::str::CharIndices<'_>,
    out: &mut String,
) -> Result<(), String> {
    let mut codepoint = String::with_capacity(JSON_UNICODE_ESCAPE_HEX_DIGITS);
    for _ in 0..JSON_UNICODE_ESCAPE_HEX_DIGITS {
        let Some((_, ch)) = chars.next() else {
            return Err("unterminated JSON unicode escape".to_string());
        };
        if !ch.is_ascii_hexdigit() {
            return Err(format!("invalid JSON unicode escape in {text:?}"));
        }
        codepoint.push(ch);
    }
    let value = u32::from_str_radix(&codepoint, JSON_HEX_RADIX)
        .map_err(|err| format!("parse JSON unicode escape: {err}"))?;
    let Some(ch) = char::from_u32(value) else {
        return Err(format!("invalid JSON unicode codepoint {codepoint}"));
    };
    out.push(ch);
    Ok(())
}

pub(crate) fn parse_json_string_array(text: &str) -> Result<Vec<String>, String> {
    let mut rest = text.trim_start();
    if !rest.starts_with('[') {
        return Err("expected JSON string array".to_string());
    }
    rest = &rest[1..];
    let mut out = Vec::new();
    loop {
        rest = rest.trim_start();
        if let Some(after) = rest.strip_prefix(']') {
            let _ = after;
            return Ok(out);
        }
        let (value, after_string) = parse_json_string(rest)?;
        out.push(value);
        rest = after_string.trim_start();
        if let Some(after) = rest.strip_prefix(',') {
            rest = after;
        } else if rest.starts_with(']') {
            continue;
        } else {
            return Err("expected comma or closing bracket in JSON string array".to_string());
        }
    }
}

pub(crate) fn json_optional_string(value: Option<&str>) -> String {
    value.map(json_string).unwrap_or_else(|| "null".to_string())
}

pub(crate) fn json_string_array(values: &[&str]) -> String {
    json_string_iter(values.iter().copied())
}

pub(crate) fn json_string_vec(values: &[String]) -> String {
    json_string_iter(values.iter().map(String::as_str))
}

pub(crate) fn json_string_iter<'a>(values: impl IntoIterator<Item = &'a str>) -> String {
    let mut out = String::from("[");
    for (idx, value) in values.into_iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        out.push_str(&json_string(value));
    }
    out.push(']');
    out
}

pub(crate) fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + JSON_STRING_QUOTE_COUNT);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch.is_control() => out.push_str(&format_json_control_escape(ch)),
            ch => out.push(ch),
        }
    }
    out.push('"');
    out
}

fn format_json_control_escape(ch: char) -> String {
    format!(
        "\\u{value:0width$x}",
        value = ch as u32,
        width = JSON_UNICODE_ESCAPE_HEX_DIGITS
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_FIELD: &str = "field";
    const SAMPLE_CONTEXT: &str = "fixture";

    #[test]
    fn json_string_round_trips_escaping_and_control_characters() {
        let value = "quote=\" slash=\\ newline=\n tab=\t nul=\0";
        let encoded = json_string(value);
        let (decoded, rest) = parse_json_string(&encoded).expect("encoded value parses");

        assert_eq!(decoded, value);
        assert!(rest.is_empty(), "{rest:?}");
    }

    #[test]
    fn json_parser_rejects_malformed_escaping() {
        let err = parse_json_string("\"bad\\xescape\"").expect_err("bad escape fails");

        assert!(err.contains("unsupported JSON escape"), "{err}");
    }

    #[test]
    fn json_field_uniqueness_rejects_duplicate_keys() {
        let duplicate = "{\"field\": \"one\", \"field\": \"two\"}";
        let err = ensure_unique_json_field(duplicate, SAMPLE_FIELD, SAMPLE_CONTEXT)
            .expect_err("duplicate key fails");

        assert!(err.contains("duplicate field field"), "{err}");
    }

    #[test]
    fn json_string_array_rejects_missing_delimiters() {
        let err =
            parse_json_string_array("[\"one\" \"two\"]").expect_err("missing comma fails closed");

        assert!(err.contains("expected comma"), "{err}");
    }
}
