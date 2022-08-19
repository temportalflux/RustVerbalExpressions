use regex::Regex;

use super::escape;
use crate::Expression as E;
use crate::Verex;

const A_VEREX_STRING: &'static str = r"(?:a)";

#[test]
fn test_escape() {
    let string = r"\()[]{}.+*?^$|";
    let escaped = escape(string);
    assert_eq!(r"\\\(\)\[\]\{\}\.\+\*\?\^\$\|", escaped);
    let regex = Regex::new(escaped.as_ref()).unwrap();
    assert!(regex.is_match(string));

    let reverse = r"|$^?*+.}{][)(\";
    let reverse_escaped = escape(reverse);
    assert_eq!(r"\|\$\^\?\*\+\.\}\{\]\[\)\(\\", reverse_escaped);
    let regex = Regex::new(reverse_escaped.as_ref()).unwrap();
    assert!(regex.is_match(reverse));
}

#[test]
fn test_constructors() {
    let verex1: Verex = Verex::new();
    assert_eq!(verex1.source(), r"(?:)");

    let verex2: Verex = Verex::from_string(r"a".to_owned());
    assert_eq!(verex2.source(), A_VEREX_STRING);

    let verex3: Verex = Verex::from_str(r"a");
    assert_eq!(verex3.source(), A_VEREX_STRING);
}

#[test]
fn test_add() {
    let mut verex: Verex = Verex::new();
    verex.add(r"a");
    verex.update_source_with_modifiers();
    assert_eq!(verex.source(), A_VEREX_STRING);
}

#[test]
fn test_update_source_with_modifiers() {
    let mut verex = Verex::new();
    verex.add(r"a");
    assert_eq!(verex.source(), r"(?:)");
    verex.update_source_with_modifiers();
    assert_eq!(verex.source(), A_VEREX_STRING);
    verex.search_one_line(false);
    assert_eq!(verex.source(), r"(?m:a)");
}

#[test]
fn test_compile_regex() {
    let mut verex: Verex = Verex::new();
    verex.find(r"a");

    let regex1 = verex.compile().unwrap();
    assert!(regex1.is_match(r"a"));

    let regex2 = verex.regex().unwrap();
    assert!(regex2.is_match(r"a"));
}

#[test]
fn test_i_modifier() {
    let mut verex = Verex::from_str(r"a");
    verex.with_any_case(true);
    assert_eq!(verex.source(), r"(?i:a)");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"a"));
    assert!(regex.is_match(r"A"));
    assert!(!regex.is_match(r"b"));
}

#[test]
fn test_m_modifier() {
    let verex = Verex::new()
                   .start_of_line()
                   .find(r"a")
                   .end_of_line()
                   .search_one_line(false)
                   .clone();
    assert_eq!(verex.source(), r"(?m:^(?:a)$)");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"a"));
    assert!(!regex.is_match(r"aa"));
    assert!(regex.is_match("a\n"));
    assert_eq!(regex.find_iter("a\na").count(), 2);
}

#[test]
fn test_source_and_raw_and_value() {
    let verex: Verex = Verex::from_str(r"a");
    assert_eq!(verex.source(), A_VEREX_STRING);
    assert_eq!(verex.raw(), A_VEREX_STRING);
    assert_eq!(verex.value(), A_VEREX_STRING);
}

#[test]
fn test_any_and_any_of() {
    let mut verex1: Verex = Verex::new();
    verex1.any(r"ab");

    let regex1 = verex1.compile().unwrap();
    assert!(regex1.is_match(r"a"));
    assert!(regex1.is_match(r"b"));
    assert!(!regex1.is_match(r"c"));

    let mut verex2: Verex = Verex::new();
    verex2.any_of(r"ab");

    let regex2 = verex2.compile().unwrap();
    assert!(regex2.is_match(r"a"));
    assert!(regex2.is_match(r"b"));
    assert!(!regex2.is_match(r"c"));
}

#[test]
fn test_anything() {
    let mut verex: Verex = Verex::new();
    verex.anything();
    assert_eq!(verex.source(), r"(?:(.*))");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r""));
    assert!(regex.is_match(r"foobar"));
}

#[test]
fn test_anything_but() {
    let mut verex: Verex = Verex::new();
    verex.start_of_line()
         .anything_but("foo")
         .end_of_line();
    assert_eq!(verex.source(), r"(?:^(?:[^foo]*)$)");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r""));
    assert!(regex.is_match(r"bar"));
    assert!(!regex.is_match(r"foo"));
    assert!(!regex.is_match(r"foofoo"));
    assert!(!regex.is_match(r"barfoo"));
}

#[test]
fn test_br_and_linebreak() {
    // br
    let or_regex_string = r"(?:(?:\n|(?:\r\n)))";
    let verex1 = Verex::new().br().clone();
    assert_eq!(verex1.source(), or_regex_string);

    let regex1 = verex1.compile().unwrap();
    assert!(regex1.is_match("\n"));
    assert!(regex1.is_match("\r\n"));

    // line_break
    let verex2 = Verex::new().line_break().clone();
    assert_eq!(verex2.source(), or_regex_string);

    let regex2 = verex2.compile().unwrap();
    assert!(regex2.is_match("\n"));
    assert!(regex2.is_match("\r\n"));
}

#[test]
fn test_capture_value() {
    let mut verex = Verex::new();
    verex.capture_value("foo");
    let text = "bar foo baz";
    let regex = verex.compile().unwrap();
    let capture = regex.captures(text).unwrap();
    assert_eq!(capture.get(1).map(|m| m.as_str()), Some("foo"));
}

#[test]
fn test_capture() {
    let mut verex = Verex::new();
    verex.capture("[a]*"); // capture a regex expression that will be escaped
    let text = "bar [a]* baz aa";
    let regex = verex.compile().unwrap();
    let capture = regex.captures(text).unwrap();
    assert_eq!(capture.get(1).map(|m| m.as_str()), Some("[a]*"));
    assert_eq!(capture.get(2).map(|m| m.as_str()), None); // regex is escaped and does not match
}

#[test]
fn test_capture_expr() {
    let mut verex = Verex::new();
    verex.capture_expr(E::String("(?:aa)+")); // capture a regex expression that will not be escaped
    let text = "bar [a]* baz aa aaaa";
    let regex = verex.compile().unwrap();
    let mut captures = regex.captures_iter(text);
    let capture1 = captures.next().unwrap();
    assert_eq!(capture1.get(1).map(|m| m.as_str()), Some("aa"));
    assert_eq!(capture1.get(2).map(|m| m.as_str()), None);
    let capture2 = captures.next().unwrap();
    assert_eq!(capture2.get(1).map(|m| m.as_str()), Some("aaaa"));
    assert_eq!(capture2.get(2).map(|m| m.as_str()), None);

    assert!(captures.next().is_none());
}

#[test]
fn test_digit() {
    let verex = Verex::new().digit().clone();
    assert_eq!(verex.source(), r"(?:\d)");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"0"));
    assert!(regex.is_match(r"1"));
    assert!(regex.is_match(r"3"));
    assert!(regex.is_match(r"9"));
    assert!(!regex.is_match(r"a"));
    assert!(!regex.is_match(r" "));
    assert!(!regex.is_match(r"?"));
}

#[test]
fn test_find_and_then() {
    let mut verex: Verex = Verex::new();
    verex.find("foo");
    assert_eq!(verex.source(), r"(?:(?:foo))");

    let regex = verex.compile().unwrap();
    assert!(!regex.is_match(r"bar"));
    assert!(regex.is_match(r"foo"));
    assert!(regex.is_match(r"foofoo"));
    assert!(regex.is_match(r"barfoo"));

    // same as find
    let mut verex2: Verex = Verex::new();
    verex2.then("foo");
    assert_eq!(verex2.source(), r"(?:(?:foo))");

    let regex2 = verex2.compile().unwrap();
    assert!(!regex2.is_match(r"bar"));
    assert!(regex2.is_match(r"foo"));
    assert!(regex2.is_match(r"foofoo"));
    assert!(regex2.is_match(r"barfoo"));
}

#[test]
fn test_find_escapes() {
    let mut verex: Verex = Verex::new();
    verex.find("[fo]+");
    assert_eq!(verex.source(), r"(?:(?:\[fo\]\+))");

    let regex = verex.compile().unwrap();
    assert!(!regex.is_match(r"foo"));
    assert!(regex.is_match(r"[fo]+"));
}

#[test]
fn test_find_chained() {
    let mut verex: Verex = Verex::new();
    verex.find("foo")
         .then("bar");
    assert_eq!(verex.source(), r"(?:(?:foo)(?:bar))");

    let regex = verex.compile().unwrap();
    assert!(!regex.is_match(r"bar"));
    assert!(!regex.is_match(r"foo"));
    assert!(!regex.is_match(r"barfoo"));
    assert!(regex.is_match(r"foobar"));
}

#[test]
fn test_find_expr_string() {
    let mut verex = Verex::new();
    verex.find_expr(E::String(r"[a-c]"));
    assert_eq!(verex.source(), r"(?:(?:[a-c]))");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"a"));
    assert!(regex.is_match(r"b"));
    assert!(regex.is_match(r"c"));
    assert!(!regex.is_match(r"d"));
}

#[test]
fn test_find_expr_verex() {
    let insert_verex = Verex::new().range(vec![('a', 'c')]).clone();
    let mut verex = Verex::new();
    verex.find_expr(E::Verex(&insert_verex));
    assert_eq!(verex.source(), r"(?:(?:(?:[a-c])))");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"a"));
    assert!(regex.is_match(r"b"));
    assert!(regex.is_match(r"c"));
    assert!(!regex.is_match(r"d"));
}

#[test]
fn test_find_expr_regex() {
    let insert_regex = Verex::new().range(vec![('a', 'c')]).compile().unwrap();
    let mut verex = Verex::new();
    verex.find_expr(E::Regex(&insert_regex));
    assert_eq!(verex.source(), r"(?:(?:(?:[a-c])))");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"a"));
    assert!(regex.is_match(r"b"));
    assert!(regex.is_match(r"c"));
    assert!(!regex.is_match(r"d"));
}

#[test]
fn test_maybe() {
    let mut verex: Verex = Verex::new();
    verex.start_of_line()
         .maybe(r"a")
         .end_of_line();
    assert_eq!(verex.source(), r"(?:^(?:a)?$)");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r""));
    assert!(regex.is_match(r"a"));
    assert!(!regex.is_match(r"foo"));
}

#[test]
fn test_maybe_expr() {
    let mut verex: Verex = Verex::new();
    verex.start_of_line()
         .maybe_expr(E::String(r"(?:a)"))
         .end_of_line();
    assert_eq!(verex.source(), r"(?:^(?:(?:a))?$)");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r""));
    assert!(regex.is_match(r"a"));
    assert!(!regex.is_match(r"foo"));
}

#[test]
fn test_or_and_or_find() {
    let mut verex1 = Verex::new();
    verex1.find(r"a")
          .or()
          .find(r"b");
    assert_eq!(verex1.source(), r"(?:(?:a)|(?:b))");

    let regex1 = verex1.compile().unwrap();
    assert!(regex1.is_match(r"a"));
    assert!(regex1.is_match(r"b"));
    assert!(!regex1.is_match(r"z"));

    let mut verex2 = Verex::new();
    verex2.find(r"a")
          .or_find(r"b");
    assert_eq!(verex2.source(), r"(?:(?:a)|(?:b))");

    let regex2 = verex2.compile().unwrap();
    assert!(regex2.is_match(r"a"));
    assert!(regex2.is_match(r"b"));
    assert!(!regex2.is_match(r"z"));
}

#[test]
fn test_range() {
    let verex = Verex::new().range(vec![('a', 'z')]).clone();
    assert_eq!(verex.source(), r"(?:[a-z])");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"a"));
    assert!(regex.is_match(r"b"));
    assert!(regex.is_match(r"h"));
    assert!(regex.is_match(r"u"));
    assert!(regex.is_match(r"z"));
    assert!(!regex.is_match(r"A"));
    assert!(!regex.is_match(r"Z"));
}

#[test]
fn test_repeat_n_and_repeat_previous() {
    // repeat_n
    let verex = Verex::new().find("a").repeat_n(3).clone();
    assert_eq!(verex.source(), r"(?:(?:a){3})");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"aaa"));
    assert!(!regex.is_match(r"a"));
    assert!(!regex.is_match(r"b"));

    // repeat_previous
    let verex2 = Verex::new().find("a").repeat_previous(3).clone();
    assert_eq!(verex2.source(), r"(?:(?:a){3})");

    let regex2 = verex2.compile().unwrap();
    assert!(regex2.is_match(r"aaa"));
    assert!(!regex2.is_match(r"a"));
    assert!(!regex2.is_match(r"b"));
}

#[test]
fn test_repeat_n_to_m() {
    let verex = Verex::new().find("a").repeat_n_to_m(3, 4).clone();
    assert_eq!(verex.source(), r"(?:(?:a){3,4})");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"aaa"));
    assert!(regex.is_match(r"aaaa"));
    assert!(!regex.is_match(r"a"));
    assert!(!regex.is_match(r"b"));
}

#[test]
fn test_repeat_once_or_more() {
    let verex = Verex::new().find("a").repeat_once_or_more().clone();
    assert_eq!(verex.source(), r"(?:(?:a)+)");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"aaa"));
    assert!(regex.is_match(r"baaa"));
    assert!(regex.is_match(r"aaaa"));
    assert!(regex.is_match(r"a"));
    assert!(!regex.is_match(r""));
    assert!(!regex.is_match(r"b"));
}

#[test]
fn test_repeat_zero_or_more() {
    let verex = Verex::new().find("b").find("a").repeat_zero_or_more().find("b").clone();
    assert_eq!(verex.source(), r"(?:(?:b)(?:a)*(?:b))");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"baaab"));
    assert!(regex.is_match(r"baaaab"));
    assert!(regex.is_match(r"bab"));
    assert!(regex.is_match(r"bb"));
    assert!(!regex.is_match(r"bacab"));
    assert!(!regex.is_match(r"bcb"));
}

#[test]
fn test_replace() {
    let verex = Verex::from_str(r"r");
    let replaced = verex.replace(r"foobar", r"z").unwrap();
    assert_eq!(replaced, r"foobaz");
}

#[test]
fn test_something() {
    let mut verex: Verex = Verex::new();
    verex.something();
    assert_eq!(verex.source(), r"(?:(.+))");

    let regex = verex.compile().unwrap();
    assert!(!regex.is_match(r""));
    assert!(regex.is_match(r"foobar"));
}

#[test]
fn test_someting_but() {
    let mut verex: Verex = Verex::new();
    verex.start_of_line()
         .something_but("foo")
         .end_of_line();
    assert_eq!(verex.source(), r"(?:^(?:[^foo]+)$)");

    let regex = verex.compile().unwrap();
    assert!(!regex.is_match(r""));
    assert!(regex.is_match(r"bar"));
    assert!(!regex.is_match(r"foo"));
    assert!(!regex.is_match(r"foofoo"));
    assert!(!regex.is_match(r"barfoo"));
}

#[test]
fn test_word() {
    let mut verex = Verex::new();
    verex.word();
    assert_eq!(verex.source(), r"(?:(?:\w+))");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"word"));
    assert!(regex.is_match(r"w0rd"));
    assert!(!regex.is_match(r"./"));
}

// test the standalone functions
