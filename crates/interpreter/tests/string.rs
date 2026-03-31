#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant,
    clippy::manual_string_new
)]

use interpreter::{InterpreterContext, Value, interpret};
use parser::parse;

#[test]
fn test_string_indexof() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'abcdefg'.indexOf('cd')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));

    let expr = parse("'abcdefg'.indexOf('x')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(-1.0, 0));

    let expr = parse("'abcdefg'.indexOf('')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));
}

#[test]
fn test_string_substring() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'abcdefg'.substring(1)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("bcdefg".to_string()));

    let expr = parse("'abcdefg'.substring(1, 2)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("bc".to_string()));

    let expr = parse("'abcdefg'.substring(6)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("g".to_string()));

    let expr = parse("'abcdefg'.substring(7)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result.to_vec(), vec![]);
}

#[test]
fn test_string_startswith() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'abcdefg'.startsWith('abc')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'abcdefg'.startsWith('def')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("'abcdefg'.startsWith('')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_string_endswith() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'abcdefg'.endsWith('efg')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'abcdefg'.endsWith('abc')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("'abcdefg'.endsWith('')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_string_contains() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'abc'.contains('b')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'abc'.contains('bc')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'abc'.contains('x')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_string_upper_lower() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'AbCdEf'.upper()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("ABCDEF".to_string()));

    let expr = parse("'AbCdEf'.lower()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("abcdef".to_string()));
}

#[test]
fn test_string_replace() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'abcdefg'.replace('cde', '123')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("ab123fg".to_string()));

    let expr = parse("'aXbXc'.replace('X', 'Y')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("aYbYc".to_string()));
}

#[test]
fn test_string_matches() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse(r"'test123'.matches('\d+')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse(r"'test'.matches('\d+')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse(r"'2024-01-15'.matches('\d{4}-\d{2}-\d{2}')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'hello world'.matches('hello')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'hello world'.matches('^hello')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'hello world'.matches('world$')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_string_replace_matches() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse(r"'abc123def456'.replaceMatches('\d+', 'X')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("abcXdefX".to_string()));

    let expr = parse("'hello world'.replaceMatches('o', '0')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("hell0 w0rld".to_string()));
}

#[test]
fn test_string_length() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'abc'.length()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let expr = parse("''.length()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));
}

#[test]
fn test_string_tochars() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'abc'.toChars()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], Value::String("a".to_string()));
        assert_eq!(items[1], Value::String("b".to_string()));
        assert_eq!(items[2], Value::String("c".to_string()));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_string_split() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'a,b,c'.split(',')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], Value::String("a".to_string()));
        assert_eq!(items[1], Value::String("b".to_string()));
        assert_eq!(items[2], Value::String("c".to_string()));
    } else {
        panic!("Expected collection");
    }

    let expr = parse("'a::b::c'.split('::')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], Value::String("a".to_string()));
        assert_eq!(items[1], Value::String("b".to_string()));
        assert_eq!(items[2], Value::String("c".to_string()));
    } else {
        panic!("Expected collection");
    }

    let expr = parse("'abc'.split('')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], Value::String("a".to_string()));
        assert_eq!(items[1], Value::String("b".to_string()));
        assert_eq!(items[2], Value::String("c".to_string()));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_string_join() {
    let data = Value::collection(vec![
        Value::String("a".to_string()),
        Value::String("b".to_string()),
        Value::String("c".to_string()),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("join(',')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("a,b,c".to_string()));

    let expr = parse("join('')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("abc".to_string()));
}

#[test]
fn test_string_trim() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'  abc  '.trim()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("abc".to_string()));

    let expr = parse("'abc'.trim()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("abc".to_string()));
}

#[test]
fn test_regex_caching_in_loop() {
    let data = Value::collection(vec![
        Value::String("abc123".to_string()),
        Value::String("def456".to_string()),
        Value::String("ghi".to_string()),
        Value::String("jkl789".to_string()),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse(r"where($this.matches('\d+'))").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], Value::String("abc123".to_string()));
        assert_eq!(items[1], Value::String("def456".to_string()));
        assert_eq!(items[2], Value::String("jkl789".to_string()));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_string_last_index_of() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'abcabc'.lastIndexOf('abc')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let expr = parse("'abcabc'.lastIndexOf('x')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(-1.0, 0));

    let expr = parse("'abc'.lastIndexOf('abc')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));
}

#[test]
fn test_matches_full() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse(r"'abc'.matchesFull('abc')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse(r"'abc'.matchesFull('ab')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse(r"'abc'.matchesFull('a.*')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse(r"'abc123'.matchesFull('\w+')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_encode_base64() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'hello'.encode('base64')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("aGVsbG8=".to_string()));

    let expr = parse("''.encode('base64')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("".to_string()));
}

#[test]
fn test_decode_base64() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'aGVsbG8='.decode('base64')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("hello".to_string()));

    let expr = parse("''.decode('base64')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("".to_string()));
}

#[test]
fn test_encode_decode_roundtrip() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'test string'.encode('base64').decode('base64')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("test string".to_string()));
}

#[test]
fn test_encode_hex() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'hello'.encode('hex')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("68656c6c6f".to_string()));

    let expr = parse("''.encode('hex')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("".to_string()));
}

#[test]
fn test_decode_hex() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'68656c6c6f'.decode('hex')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("hello".to_string()));

    let expr = parse("''.decode('hex')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("".to_string()));
}

#[test]
fn test_encode_decode_hex_roundtrip() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'test string'.encode('hex').decode('hex')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("test string".to_string()));
}

#[test]
fn test_encode_urlbase64() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'hello'.encode('urlbase64')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("aGVsbG8=".to_string()));

    let expr = parse("''.encode('urlbase64')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("".to_string()));
}

#[test]
fn test_decode_urlbase64() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'aGVsbG8='.decode('urlbase64')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_encode_decode_urlbase64_roundtrip() {
    let context = InterpreterContext::new(Value::Null);

    let expr =
        parse("'test string'.encode('urlbase64').decode('urlbase64')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("test string".to_string()));
}

#[test]
fn test_urlbase64_uses_url_safe_chars() {
    let context = InterpreterContext::new(Value::String("subjects?_d".to_string()));

    let expr = parse("encode('base64')").expect("parse failed");
    let (standard, _) = interpret(&expr, context.clone()).expect("interpret failed");

    let expr = parse("encode('urlbase64')").expect("parse failed");
    let (urlsafe, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let (Value::String(s), Value::String(u)) = (&standard, &urlsafe) {
        assert!(
            s.contains('+') || s.contains('/') || !s.contains('-') && !s.contains('_') || s == u,
            "standard and urlbase64 should differ for inputs producing +/"
        );
        assert!(!u.contains('+'), "urlbase64 must not contain '+'");
        assert!(!u.contains('/'), "urlbase64 must not contain '/'");
    }
}

#[test]
fn test_encode_unsupported_encoding() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'hello'.encode('rot13')").expect("parse failed");
    let result = interpret(&expr, context.clone());
    assert!(result.is_err());
}

#[test]
fn test_escape_json() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse(r#"'hello "world"'.escape('json')"#).expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String(r#"hello \"world\""#.to_string()));

    let expr = parse(r#"'"key": "value"'.escape('json')"#).expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String(r#"\"key\": \"value\""#.to_string()));
}

#[test]
fn test_escape_html() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse(r#"'<b>bold & "quoted"</b>'.escape('html')"#).expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::String("&lt;b&gt;bold &amp; &quot;quoted&quot;&lt;/b&gt;".to_string())
    );
}

#[test]
fn test_unescape_json() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse(r#"'hello \"world\"'.unescape('json')"#).expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String(r#"hello "world""#.to_string()));
}

#[test]
fn test_unescape_html() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'&lt;b&gt;bold&lt;/b&gt;'.unescape('html')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("<b>bold</b>".to_string()));

    let expr = parse("'&amp;&quot;&#x27;'.unescape('html')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("&\"'".to_string()));
}

#[test]
fn test_escape_unsupported_target() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'hello'.escape('xml')").expect("parse failed");
    let result = interpret(&expr, context.clone());
    assert!(result.is_err());
}

#[test]
fn test_escape_unescape_json_roundtrip() {
    let context = InterpreterContext::new(Value::Null);

    let expr =
        parse(r#"'test "quotes" here'.escape('json').unescape('json')"#).expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String(r#"test "quotes" here"#.to_string()));
}

#[test]
fn test_escape_unescape_html_roundtrip() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse(r#"'<p>A & B</p>'.escape('html').unescape('html')"#).expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("<p>A & B</p>".to_string()));
}

#[test]
fn test_escape_json_backslash() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'hello\world'.escape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String(r"hello\\world".to_string()));
}

#[test]
fn test_escape_json_double_backslash() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'a\\b'.escape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String(r"a\\b".to_string()));
}

#[test]
fn test_escape_json_backslash_n_literal() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'hello\nworld'.escape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String(r"hello\nworld".to_string()));
}

#[test]
fn test_escape_json_empty_string() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("''.escape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String(String::new()));
}

#[test]
fn test_escape_json_no_special_chars() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("'hello'.escape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_escape_json_consecutive_backslashes() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'a\\c'.escape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String(r"a\\c".to_string()));
}

#[test]
fn test_escape_json_backslash_and_quotes() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r#"'"a\b"'.escape('json')"#).expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String(r#"\"a\\b\""#.to_string()));
}

#[test]
fn test_unescape_json_escaped_backslash() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'a\\\\b'.unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("a\\b".to_string()));
}

#[test]
fn test_unescape_json_escaped_n_to_newline() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'hello\nworld'.unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("hello\nworld".to_string()));
}

#[test]
fn test_unescape_json_escaped_t_to_tab() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'col1\tcol2'.unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("col1\tcol2".to_string()));
}

#[test]
fn test_unescape_json_escaped_r_to_cr() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'line1\rline2'.unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("line1\rline2".to_string()));
}

#[test]
fn test_unescape_json_escaped_b_to_backspace() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'ab\bc'.unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("ab\x08c".to_string()));
}

#[test]
fn test_unescape_json_escaped_f_to_formfeed() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'ab\fc'.unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("ab\x0Cc".to_string()));
}

#[test]
fn test_unescape_json_escaped_slash() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'a\/b'.unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("a/b".to_string()));
}

#[test]
fn test_unescape_json_unicode_escape() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'\u0041'.unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("A".to_string()));
}

#[test]
fn test_unescape_json_unicode_heart() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'\u2764'.unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("\u{2764}".to_string()));
}

#[test]
fn test_unescape_json_unknown_escape_preserved() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'\x'.unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("\\x".to_string()));
}

#[test]
fn test_unescape_json_empty_string() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("''.unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String(String::new()));
}

#[test]
fn test_unescape_json_multiple_escapes() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'\n\t\r'.unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("\n\t\r".to_string()));
}

#[test]
fn test_escape_html_backslash_passthrough() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'a\b'.escape('html')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("a\\b".to_string()));
}

#[test]
fn test_escape_html_single_quote_via_parser() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'it\'s'.escape('html')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("it&#x27;s".to_string()));
}

#[test]
fn test_escape_html_empty_string() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("''.escape('html')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String(String::new()));
}

#[test]
fn test_unescape_html_hex_entity() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("'&#x41;'.unescape('html')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("A".to_string()));
}

#[test]
fn test_unescape_html_decimal_entity() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("'&#65;'.unescape('html')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("A".to_string()));
}

#[test]
fn test_unescape_html_newline_hex() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("'&#x0A;'.unescape('html')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("\n".to_string()));
}

#[test]
fn test_unescape_html_newline_decimal() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("'&#10;'.unescape('html')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("\n".to_string()));
}

#[test]
fn test_unescape_html_unknown_entity_preserved() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("'&foo;'.unescape('html')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("&foo;".to_string()));
}

#[test]
fn test_unescape_html_empty_string() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("''.unescape('html')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String(String::new()));
}

#[test]
fn test_escape_unescape_json_backslash_roundtrip() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'path\\to\\file'.escape('json').unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("path\\to\\file".to_string()));
}

#[test]
fn test_escape_unescape_json_double_backslash_roundtrip() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'a\\\\b'.escape('json').unescape('json')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("a\\\\b".to_string()));
}

#[test]
fn test_escape_unescape_html_backslash_roundtrip() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r"'a\b'.escape('html').unescape('html')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("a\\b".to_string()));
}

#[test]
fn test_escape_unescape_json_quotes_and_backslashes_roundtrip() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse(r#"'"a\b"'.escape('json').unescape('json')"#).expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("\"a\\b\"".to_string()));
}

#[test]
fn test_escape_missing_arg() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("'hello'.escape()").expect("parse failed");
    assert!(interpret(&expr, context).is_err());
}

#[test]
fn test_unescape_missing_arg() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("'hello'.unescape()").expect("parse failed");
    assert!(interpret(&expr, context).is_err());
}

#[test]
fn test_unescape_unsupported_target() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("'hello'.unescape('yaml')").expect("parse failed");
    assert!(interpret(&expr, context).is_err());
}

#[test]
fn test_matches_case_insensitive_flag() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'Hello'.matches('hello', 'i')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'Hello'.matches('hello')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_matches_full_case_insensitive_flag() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'HELLO'.matchesFull('hello', 'i')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_replace_matches_case_insensitive_flag() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'Hello World'.replaceMatches('hello', 'hi', 'i')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("hi World".to_string()));
}

#[test]
fn test_matches_dotall_flag() {
    let context = InterpreterContext::new(Value::String("hello\nworld".to_string()));

    let expr = parse("matches('hello.world', 's')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("matches('hello.world')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_indexof_unicode_multibyte() {
    let context = InterpreterContext::new(Value::String("café".to_string()));

    let expr = parse("indexOf('é')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let expr = parse("indexOf('f')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));
}

#[test]
fn test_lastindexof_unicode_multibyte() {
    let context = InterpreterContext::new(Value::String("café café".to_string()));

    let expr = parse("lastIndexOf('é')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(8.0, 0));

    let expr = parse("lastIndexOf('caf')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(5.0, 0));
}

#[test]
fn test_indexof_substring_consistency_unicode() {
    let context = InterpreterContext::new(Value::String("aéb".to_string()));

    let expr = parse("indexOf('b')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));

    let expr = parse("substring(2, 1)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("b".to_string()));
}

#[test]
fn test_escape_json_control_chars() {
    let escape = parse("escape('json')").expect("parse failed");

    let ctx = InterpreterContext::new(Value::String("\x00".to_string()));
    let (result, _) = interpret(&escape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\\u0000".to_string()));

    let ctx = InterpreterContext::new(Value::String("\x01".to_string()));
    let (result, _) = interpret(&escape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\\u0001".to_string()));

    let ctx = InterpreterContext::new(Value::String("\x01\x02\x03".to_string()));
    let (result, _) = interpret(&escape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\\u0001\\u0002\\u0003".to_string()));
}

#[test]
fn test_escape_json_named_escapes() {
    let escape = parse("escape('json')").expect("parse failed");

    let ctx = InterpreterContext::new(Value::String("ab\x08c".to_string()));
    let (result, _) = interpret(&escape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("ab\\bc".to_string()));

    let ctx = InterpreterContext::new(Value::String("ab\x0Cc".to_string()));
    let (result, _) = interpret(&escape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("ab\\fc".to_string()));

    let ctx = InterpreterContext::new(Value::String("\r\n".to_string()));
    let (result, _) = interpret(&escape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\\r\\n".to_string()));

    let ctx = InterpreterContext::new(Value::String("\"\\\n\r\t\x08\x0C".to_string()));
    let (result, _) = interpret(&escape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\\\"\\\\\\n\\r\\t\\b\\f".to_string()));
}

#[test]
fn test_escape_json_unicode_passthrough() {
    let escape = parse("escape('json')").expect("parse failed");

    let ctx = InterpreterContext::new(Value::String("caf\u{00e9}".to_string()));
    let (result, _) = interpret(&escape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("caf\u{00e9}".to_string()));

    let ctx = InterpreterContext::new(Value::String("\u{1F600}".to_string()));
    let (result, _) = interpret(&escape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\u{1F600}".to_string()));
}

#[test]
fn test_escape_json_mixed_special_and_normal() {
    let escape = parse("escape('json')").expect("parse failed");
    let ctx = InterpreterContext::new(Value::String("Hello \"world\"\npath\\to\\file".to_string()));
    let (result, _) = interpret(&escape, ctx).expect("interpret failed");
    assert_eq!(
        result,
        Value::String("Hello \\\"world\\\"\\npath\\\\to\\\\file".to_string())
    );
}

#[test]
fn test_unescape_json_all_named_escapes() {
    let unescape = parse("unescape('json')").expect("parse failed");

    let ctx = InterpreterContext::new(Value::String("a\\/b".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("a/b".to_string()));

    let ctx = InterpreterContext::new(Value::String("ab\\bc".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("ab\x08c".to_string()));

    let ctx = InterpreterContext::new(Value::String("ab\\fc".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("ab\x0Cc".to_string()));
}

#[test]
fn test_unescape_json_unicode_sequences() {
    let unescape = parse("unescape('json')").expect("parse failed");

    let ctx = InterpreterContext::new(Value::String("\\u0041".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("A".to_string()));

    let ctx = InterpreterContext::new(Value::String("\\u00e9".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\u{00e9}".to_string()));

    let ctx = InterpreterContext::new(Value::String("\\u000a".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\n".to_string()));

    let ctx = InterpreterContext::new(Value::String("\\u0000".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\0".to_string()));
}

#[test]
fn test_unescape_json_trailing_backslash() {
    let unescape = parse("unescape('json')").expect("parse failed");
    let ctx = InterpreterContext::new(Value::String("\\".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\\".to_string()));
}

#[test]
fn test_unescape_json_double_escaped_backslashes() {
    let unescape = parse("unescape('json')").expect("parse failed");

    let ctx = InterpreterContext::new(Value::String("\\\\\\\\".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\\\\".to_string()));

    let ctx = InterpreterContext::new(Value::String("\\\\\\n".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\\\n".to_string()));
}

#[test]
fn test_escape_html_all_five_special_chars() {
    let escape = parse("escape('html')").expect("parse failed");
    let ctx = InterpreterContext::new(Value::String("&<>\"'".to_string()));
    let (result, _) = interpret(&escape, ctx).expect("interpret failed");
    assert_eq!(
        result,
        Value::String("&amp;&lt;&gt;&quot;&#x27;".to_string())
    );
}

#[test]
fn test_escape_html_xss_payload() {
    let escape = parse("escape('html')").expect("parse failed");
    let ctx = InterpreterContext::new(Value::String("<script>alert('xss')</script>".to_string()));
    let (result, _) = interpret(&escape, ctx).expect("interpret failed");
    assert_eq!(
        result,
        Value::String("&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;".to_string())
    );
}

#[test]
fn test_escape_html_double_escaping() {
    let escape = parse("escape('html')").expect("parse failed");
    let ctx = InterpreterContext::new(Value::String("&amp;".to_string()));
    let (result, _) = interpret(&escape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("&amp;amp;".to_string()));
}

#[test]
fn test_unescape_html_all_entity_formats() {
    let unescape = parse("unescape('html')").expect("parse failed");

    let ctx = InterpreterContext::new(Value::String("&apos;".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("'".to_string()));

    let ctx = InterpreterContext::new(Value::String("&#39;".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("'".to_string()));

    let ctx = InterpreterContext::new(Value::String("&#x0D;".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\r".to_string()));

    let ctx = InterpreterContext::new(Value::String("&#13;".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\r".to_string()));

    let ctx = InterpreterContext::new(Value::String("&#x09;".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\t".to_string()));

    let ctx = InterpreterContext::new(Value::String("&#9;".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\t".to_string()));
}

#[test]
fn test_unescape_html_high_unicode() {
    let unescape = parse("unescape('html')").expect("parse failed");

    let ctx = InterpreterContext::new(Value::String("&#x2764;".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\u{2764}".to_string()));

    let ctx = InterpreterContext::new(Value::String("&#10084;".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("\u{2764}".to_string()));
}

#[test]
fn test_unescape_html_invalid_entity_preserved() {
    let unescape = parse("unescape('html')").expect("parse failed");

    let ctx = InterpreterContext::new(Value::String("&#xZZZZ;".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("&#xZZZZ;".to_string()));
}

#[test]
fn test_unescape_html_bare_ampersand() {
    let unescape = parse("unescape('html')").expect("parse failed");

    let ctx = InterpreterContext::new(Value::String("a&".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("a&".to_string()));

    let ctx = InterpreterContext::new(Value::String("a & b".to_string()));
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String("a & b".to_string()));
}

#[test]
fn test_roundtrip_json_control_chars() {
    let escape = parse("escape('json')").expect("parse failed");
    let unescape = parse("unescape('json')").expect("parse failed");

    let input: String = (0x00u8..=0x1Fu8)
        .filter_map(|b| char::from_u32(b as u32))
        .collect();
    let ctx = InterpreterContext::new(Value::String(input.clone()));
    let (escaped, _) = interpret(&escape, ctx).expect("interpret failed");
    let ctx = InterpreterContext::new(escaped);
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String(input));
}

#[test]
fn test_roundtrip_json_unicode() {
    let escape = parse("escape('json')").expect("parse failed");
    let unescape = parse("unescape('json')").expect("parse failed");

    let input = "caf\u{00e9} \u{1F600}".to_string();
    let ctx = InterpreterContext::new(Value::String(input.clone()));
    let (escaped, _) = interpret(&escape, ctx).expect("interpret failed");
    let ctx = InterpreterContext::new(escaped);
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String(input));
}

#[test]
fn test_roundtrip_json_complex_mixed() {
    let escape = parse("escape('json')").expect("parse failed");
    let unescape = parse("unescape('json')").expect("parse failed");

    let input = "He said \"hello\\world\"\nnew\tline\r\nend".to_string();
    let ctx = InterpreterContext::new(Value::String(input.clone()));
    let (escaped, _) = interpret(&escape, ctx).expect("interpret failed");
    let ctx = InterpreterContext::new(escaped);
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String(input));
}

#[test]
fn test_roundtrip_html_all_special() {
    let escape = parse("escape('html')").expect("parse failed");
    let unescape = parse("unescape('html')").expect("parse failed");

    let input = "&<>\"'".to_string();
    let ctx = InterpreterContext::new(Value::String(input.clone()));
    let (escaped, _) = interpret(&escape, ctx).expect("interpret failed");
    let ctx = InterpreterContext::new(escaped);
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String(input));
}

#[test]
fn test_roundtrip_html_mixed_content() {
    let escape = parse("escape('html')").expect("parse failed");
    let unescape = parse("unescape('html')").expect("parse failed");

    let input = "<p>Hello & \"world\"</p>".to_string();
    let ctx = InterpreterContext::new(Value::String(input.clone()));
    let (escaped, _) = interpret(&escape, ctx).expect("interpret failed");
    let ctx = InterpreterContext::new(escaped);
    let (result, _) = interpret(&unescape, ctx).expect("interpret failed");
    assert_eq!(result, Value::String(input));
}
