use crate::InterpreterResult;
use crate::context::InterpreterContext;
use crate::error::InterpreterError;
use crate::value::Value;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static REGEX_CACHE: RefCell<HashMap<String, Regex>> = RefCell::new(HashMap::new());
}

fn get_cached_regex(pattern: &str, flags: Option<String>) -> Result<Regex, InterpreterError> {
    let flags = flags.unwrap_or_else(|| "s".to_string());
    let pattern_with_flags = format!("(?{flags}){pattern}");
    REGEX_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(regex) = cache.get(&pattern_with_flags) {
            return Ok(regex.clone());
        }
        let regex = Regex::new(&pattern_with_flags)
            .map_err(|e| InterpreterError::InvalidRegex(e.to_string()))?;
        cache.insert(pattern_with_flags, regex.clone());
        Ok(regex)
    })
}

#[allow(clippy::cast_precision_loss)]
pub fn index_of(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let s = match base.as_string() {
        Some(s) => s,
        None => return Ok((Value::collection(vec![]), context)),
    };
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "indexOf() requires a substring argument".to_string(),
        ));
    }
    let substring = match args[0].as_string() {
        Some(s) => s,
        None => return Ok((Value::collection(vec![]), context)),
    };
    let char_index = s
        .find(&substring)
        .map_or(-1.0, |byte_idx| s[..byte_idx].chars().count() as f64);
    Ok((Value::Number(char_index, 0), context))
}

pub fn substring(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let s = match base.as_string() {
        Some(s) => s,
        None => return Ok((Value::collection(vec![]), context)),
    };
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "substring() requires a start argument".to_string(),
        ));
    }
    let start_f64 = match args[0].to_f64() {
        Some(v) => v,
        None => return Ok((Value::collection(vec![]), context)),
    };

    if start_f64 < 0.0 {
        return Ok((Value::collection(vec![]), context));
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let start = start_f64.trunc() as usize;

    let chars: Vec<char> = s.chars().collect();
    if start >= chars.len() {
        return Ok((Value::collection(vec![]), context));
    }

    let result = if args.len() > 1 {
        let length = match args[1].to_usize() {
            Some(v) => v,
            None => return Ok((Value::collection(vec![]), context)),
        };
        chars[start..].iter().take(length).collect()
    } else {
        chars[start..].iter().collect()
    };
    Ok((Value::String(result), context))
}

pub fn starts_with(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let s = match base.as_string() {
        Some(s) => s,
        None => return Ok((Value::collection(vec![]), context)),
    };
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "startsWith() requires a prefix argument".to_string(),
        ));
    }
    let prefix = match args[0].as_string() {
        Some(s) => s,
        None => return Ok((Value::collection(vec![]), context)),
    };
    Ok((Value::Boolean(s.starts_with(&prefix)), context))
}

pub fn ends_with(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let s = match base.as_string() {
        Some(s) => s,
        None => return Ok((Value::collection(vec![]), context)),
    };
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "endsWith() requires a suffix argument".to_string(),
        ));
    }
    let suffix = match args[0].as_string() {
        Some(s) => s,
        None => return Ok((Value::collection(vec![]), context)),
    };
    Ok((Value::Boolean(s.ends_with(&suffix)), context))
}

pub fn contains(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let s = match base.as_string() {
        Some(s) => s,
        None => return Ok((Value::collection(vec![]), context)),
    };
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "contains() requires a substring argument".to_string(),
        ));
    }
    let substring = match args[0].as_string() {
        Some(s) => s,
        None => return Ok((Value::collection(vec![]), context)),
    };
    Ok((Value::Boolean(s.contains(&substring)), context))
}

pub fn upper(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let s = base.to_str()?;
    Ok((Value::String(s.to_uppercase()), context))
}

pub fn lower(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let s = base.to_str()?;
    Ok((Value::String(s.to_lowercase()), context))
}

pub fn replace(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let s = base.to_str()?;
    if args.len() < 2 {
        return Err(InterpreterError::InvalidOperation(
            "replace() requires pattern and substitution arguments".to_string(),
        ));
    }
    let pattern = args[0].to_str()?;
    let substitution = args[1].to_str()?;
    Ok((Value::String(s.replace(&pattern, &substitution)), context))
}

pub fn matches(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let s = base.to_str()?;
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "matches() requires a regex argument".to_string(),
        ));
    }
    let pattern = args[0].to_str()?;
    let flags = if args.len() >= 2 {
        Some(args[1].to_str()?)
    } else {
        None
    };
    let regex = get_cached_regex(&pattern, flags)?;
    Ok((Value::Boolean(regex.is_match(&s)), context))
}

pub fn replace_matches(
    base: &Value,
    args: &[Value],
    context: InterpreterContext,
) -> InterpreterResult {
    let s = base.to_str()?;
    if args.len() < 2 {
        return Err(InterpreterError::InvalidOperation(
            "replaceMatches() requires regex and substitution arguments".to_string(),
        ));
    }
    let pattern = args[0].to_str()?;
    let substitution = args[1].to_str()?;
    let flags = if args.len() >= 3 {
        Some(args[2].to_str()?)
    } else {
        None
    };
    if pattern.is_empty() {
        return Ok((Value::String(s), context));
    }
    let regex = get_cached_regex(&pattern, flags)?;
    Ok((
        Value::String(regex.replace_all(&s, &substitution).into_owned()),
        context,
    ))
}

pub fn length(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let s = match base.as_string() {
        Some(s) => s,
        None => return Ok((Value::collection(vec![]), context)),
    };
    #[allow(clippy::cast_precision_loss)]
    let len = s.chars().count() as f64;
    Ok((Value::Number(len, 0), context))
}

pub fn to_chars(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let s = base.to_str()?;
    let chars: Vec<Value> = s.chars().map(|c| Value::String(c.to_string())).collect();
    Ok((Value::collection(chars), context))
}

pub fn split(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let s = base.to_str()?;
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "split() requires a separator argument".to_string(),
        ));
    }
    let separator = args[0].to_str()?;
    let parts: Vec<Value> = if separator.is_empty() {
        s.chars().map(|c| Value::String(c.to_string())).collect()
    } else {
        s.split(&separator)
            .map(|p| Value::String(p.to_string()))
            .collect()
    };
    Ok((Value::collection(parts), context))
}

pub fn join(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let items = match base {
        Value::Collection(items) => items,
        Value::Null => return Ok((Value::String(String::new()), context)),
        _ => return Ok((Value::String(base.to_string()), context)),
    };

    let separator = if args.is_empty() {
        String::new()
    } else {
        args[0].to_str()?
    };

    let strings: Vec<String> = items
        .iter()
        .filter_map(|v| match v {
            Value::String(s) => Some(s.clone()),
            Value::Null => None,
            other => Some(other.to_string()),
        })
        .collect();
    Ok((Value::String(strings.join(&separator)), context))
}

pub fn trim(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let s = base.to_str()?;
    Ok((Value::String(s.trim().to_string()), context))
}

#[allow(clippy::cast_precision_loss)]
pub fn last_index_of(
    base: &Value,
    args: &[Value],
    context: InterpreterContext,
) -> InterpreterResult {
    let s = base.to_str()?;
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "lastIndexOf() requires a substring argument".to_string(),
        ));
    }
    let substring = args[0].to_str()?;
    let char_index = s
        .rfind(&substring)
        .map_or(-1.0, |byte_idx| s[..byte_idx].chars().count() as f64);
    Ok((Value::Number(char_index, 0), context))
}

pub fn matches_full(
    base: &Value,
    args: &[Value],
    context: InterpreterContext,
) -> InterpreterResult {
    let s = base.to_str()?;
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "matchesFull() requires a regex argument".to_string(),
        ));
    }
    let pattern = args[0].to_str()?;
    let flags = if args.len() >= 2 {
        Some(args[1].to_str()?)
    } else {
        None
    };
    let anchored = format!("^(?:{})$", pattern);
    let regex = get_cached_regex(&anchored, flags)?;
    Ok((Value::Boolean(regex.is_match(&s)), context))
}

pub fn escape(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let s = base.to_str()?;
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "escape() requires a target argument".to_string(),
        ));
    }
    let target = args[0].to_str()?;
    let escaped = match target.as_str() {
        "json" => escape_json(&s),
        "html" => escape_html(&s),
        _ => {
            return Err(InterpreterError::InvalidOperation(format!(
                "escape() supports 'json' and 'html' targets, got '{}'",
                target
            )));
        }
    };
    Ok((Value::String(escaped), context))
}

pub fn unescape(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let s = base.to_str()?;
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "unescape() requires a target argument".to_string(),
        ));
    }
    let target = args[0].to_str()?;
    let unescaped = match target.as_str() {
        "json" => unescape_json(&s),
        "html" => unescape_html(&s),
        _ => {
            return Err(InterpreterError::InvalidOperation(format!(
                "unescape() supports 'json' and 'html' targets, got '{}'",
                target
            )));
        }
    };
    Ok((Value::String(unescaped), context))
}

fn escape_json(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\x08' => result.push_str("\\b"),
            '\x0C' => result.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}

#[allow(clippy::match_same_arms)]
fn unescape_json(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('/') => result.push('/'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('b') => result.push('\x08'),
                Some('f') => result.push('\x0C'),
                Some('u') => {
                    let hex: String = chars.by_ref().take(4).collect();
                    if let Ok(code) = u32::from_str_radix(&hex, 16)
                        && let Some(ch) = char::from_u32(code)
                    {
                        result.push(ch);
                    }
                }
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn escape_html(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&#x27;"),
            c => result.push(c),
        }
    }
    result
}

fn unescape_html(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' {
            let mut entity = String::new();
            let mut found_semicolon = false;
            for ec in chars.by_ref() {
                if ec == ';' {
                    found_semicolon = true;
                    break;
                }
                entity.push(ec);
            }
            if !found_semicolon {
                result.push('&');
                result.push_str(&entity);
            } else {
                match entity.as_str() {
                    "amp" => result.push('&'),
                    "lt" => result.push('<'),
                    "gt" => result.push('>'),
                    "quot" => result.push('"'),
                    "#x27" | "#39" | "apos" => result.push('\''),
                    "#x0A" | "#10" => result.push('\n'),
                    "#x0D" | "#13" => result.push('\r'),
                    "#x09" | "#9" => result.push('\t'),
                    other => {
                        if let Some(stripped) = other.strip_prefix("#x")
                            && let Ok(code) = u32::from_str_radix(stripped, 16)
                            && let Some(ch) = char::from_u32(code)
                        {
                            result.push(ch);
                        } else if let Some(stripped) = other.strip_prefix('#')
                            && let Ok(code) = stripped.parse::<u32>()
                            && let Some(ch) = char::from_u32(code)
                        {
                            result.push(ch);
                        } else {
                            result.push('&');
                            result.push_str(other);
                            result.push(';');
                        }
                    }
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn encode_hex(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        result.push(char::from_digit((b >> 4) as u32, 16).unwrap_or('0'));
        result.push(char::from_digit((b & 0x0f) as u32, 16).unwrap_or('0'));
    }
    result
}

fn decode_hex(s: &str) -> Result<Vec<u8>, InterpreterError> {
    if !s.len().is_multiple_of(2) {
        return Err(InterpreterError::InvalidOperation(
            "Failed to decode hex: odd-length string".to_string(),
        ));
    }
    let mut bytes = Vec::with_capacity(s.len() / 2);
    let mut chars = s.chars();
    while let Some(hi) = chars.next() {
        let lo = chars.next().ok_or_else(|| {
            InterpreterError::InvalidOperation("Failed to decode hex: unexpected end".to_string())
        })?;
        #[allow(clippy::cast_possible_truncation)]
        let hi = hi.to_digit(16).ok_or_else(|| {
            InterpreterError::InvalidOperation(format!(
                "Failed to decode hex: invalid character '{hi}'"
            ))
        })? as u8;
        #[allow(clippy::cast_possible_truncation)]
        let lo = lo.to_digit(16).ok_or_else(|| {
            InterpreterError::InvalidOperation(format!(
                "Failed to decode hex: invalid character '{lo}'"
            ))
        })? as u8;
        bytes.push((hi << 4) | lo);
    }
    Ok(bytes)
}

pub fn encode(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    use base64::{
        Engine as _,
        engine::general_purpose::{STANDARD, URL_SAFE},
    };

    let s = base.to_str()?;
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "encode() requires an encoding argument".to_string(),
        ));
    }
    let encoding = args[0].to_str()?;
    let encoded = match encoding.as_str() {
        "hex" => encode_hex(s.as_bytes()),
        "base64" => STANDARD.encode(s.as_bytes()),
        "urlbase64" => URL_SAFE.encode(s.as_bytes()),
        _ => {
            return Err(InterpreterError::InvalidOperation(format!(
                "encode() supports 'hex', 'base64', and 'urlbase64' encodings, got '{}'",
                encoding
            )));
        }
    };
    Ok((Value::String(encoded), context))
}

pub fn decode(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    use base64::{
        Engine as _,
        engine::general_purpose::{STANDARD, URL_SAFE},
    };

    let s = base.to_str()?;
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "decode() requires an encoding argument".to_string(),
        ));
    }
    let encoding = args[0].to_str()?;
    let bytes = match encoding.as_str() {
        "hex" => decode_hex(&s)?,
        "base64" => STANDARD.decode(s.as_bytes()).map_err(|e| {
            InterpreterError::InvalidOperation(format!("Failed to decode base64: {}", e))
        })?,
        "urlbase64" => URL_SAFE.decode(s.as_bytes()).map_err(|e| {
            InterpreterError::InvalidOperation(format!("Failed to decode urlbase64: {}", e))
        })?,
        _ => {
            return Err(InterpreterError::InvalidOperation(format!(
                "decode() supports 'hex', 'base64', and 'urlbase64' encodings, got '{}'",
                encoding
            )));
        }
    };
    let decoded = String::from_utf8(bytes).map_err(|e| {
        InterpreterError::InvalidOperation(format!(
            "Decoded {} is not valid UTF-8: {}",
            encoding, e
        ))
    })?;
    Ok((Value::String(decoded), context))
}
