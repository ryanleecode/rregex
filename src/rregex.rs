use crate::utils::{error, result, ToJs};
use crate::{set, JsArray, JsObject};
use regex;
use regex_syntax::{hir, Parser};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

/// A compiled regular expression for matching Unicode strings.
#[wasm_bindgen]
pub struct RRegExp {
  regex: regex::Regex,
}

#[wasm_bindgen]
impl RRegExp {

  /// Compiles a regular expression. Once compiled, it can be used repeatedly to search, split or replace text in a string.
  /// If an invalid expression is given, then an error is returned.
  #[wasm_bindgen(constructor)]
  pub fn new(re: &str) -> Result<RRegExp, JsValue> {
    match regex::Regex::new(re) {
      Err(e) => Err(error(e)),
      Ok(regex) => Ok(RRegExp { regex }),
    }
  }

  /// Returns true if and only if there is a match for the regex in the string given.
  /// It is recommended to use this method if all you need to do is test a match, since the underlying matching engine may be able to do less work.
  /// @see https://docs.rs/regex/latest/regex/struct.Regex.html#method.find
  #[wasm_bindgen(js_name = isMatch)]
  pub fn is_match(&self, text: &str) -> bool {
    self.regex.is_match(text)
  }

  /// Returns the same as is_match, but starts the search at the given offset.
  /// The significance of the starting point is that it takes the surrounding context into consideration. For example, the `\A` anchor can only match when `start == 0`.
  /// https://docs.rs/regex/latest/regex/struct.Regex.html#method.is_match_at
  #[wasm_bindgen(js_name = isMatchAt)]
  pub fn is_match_at(&self, text: &str, start: usize) -> bool {
    if text.len() < start {
      false
    } else {
      self.regex.is_match_at(text, start)
    }
  }

  /// Returns the start and end byte range of the leftmost-first match in `text`. If no match exists, then `null` is returned.
  /// Note that this should only be used if you want to discover the position of the match. Testing the existence of a match is faster if you use `is_match`.
  /// https://docs.rs/regex/latest/regex/struct.Regex.html#method.find
  pub fn find(&self, text: &str) -> JsValue {
    self.regex.find(text).to_js()
  }

  /// Returns the same as find, but starts the search at the given offset.
  /// The significance of the starting point is that it takes the surrounding context into consideration. For example, the \A anchor can only match when `start == 0`.
  /// @see https://docs.rs/regex/latest/regex/struct.Regex.html#method.find_at
  #[wasm_bindgen(js_name = findAt)]
  pub fn find_at(&self, text: &str, start: usize) -> JsValue {
    if text.len() < start {
      JsValue::UNDEFINED
    } else {
      self.regex.find_at(text, start).to_js()
    }
  }

  /// Returns an array for each successive non-overlapping match in text, returning the start and end byte indices with respect to text.
  /// @see https://docs.rs/regex/latest/regex/struct.Regex.html#method.find_iter
  #[wasm_bindgen(js_name = findAll)]
  pub fn find_all(&self, text: &str) -> JsValue {
    let matches: Vec<regex::Match> = self.regex.find_iter(text).collect();
    matches.to_js()
  }


  /// Replaces the leftmost-first match with the replacement provided.
  /// The replacement can be a regular string (where `$N` and `$name` are expanded to match capture groups) or a function that takes the matches’ `Captures` and returns the replaced string.
  /// If no match is found, then a copy of the string is returned unchanged.
  /// @see https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace
  pub fn replace(&self, text: &str, rep: &str) -> String {
    self.regex.replace(text, rep).into_owned()
  }

  /// Replaces at most `limit` non-overlapping matches in `text` with the replacement provided. If `limit` is 0, then all non-overlapping matches are replaced.
  /// See the documentation for `replace` for details on how to access capturing group matches in the replacement string.
  /// @see https://docs.rs/regex/latest/regex/struct.Regex.html#method.replacen
  pub fn replacen(&self, text: &str, limit: usize, rep: &str) -> String {
    self.regex.replacen(text, limit, rep).into_owned()
  }

  /// Replaces all non-overlapping matches in `text` with the replacement provided. This is the same as calling `replacen` with `limit` set to `0`.
  /// See the documentation for `replace` for details on how to access capturing group matches in the replacement string.
  /// @see https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace_all
  #[wasm_bindgen(js_name = replaceAll)]
  pub fn replace_all(&self, text: &str, rep: &str) -> String {
    self.regex.replace_all(text, rep).into_owned()
  }

  /// Returns an iterator of substrings of `text` delimited by a match of the regular expression. Namely, each element of the iterator corresponds to text that isn’t matched by the regular expression.
  /// This method will not copy the text given.
  /// @see https://docs.rs/regex/latest/regex/struct.Regex.html#method.split
  pub fn split(&self, text: &str) -> JsValue {
    let splits: Vec<String> = self.regex.split(text).map(|s| s.to_string()).collect();
    splits.to_js()
  }

  /// Returns an iterator of at most `limit` substrings of `text` delimited by a match of the regular expression. (A limit of 0 will return no substrings.) Namely, each element of the iterator corresponds to text that isn’t matched by the regular expression. The remainder of the string that is not split will be the last element in the iterator.
  /// This method will not copy the text given.
  /// @see https://docs.rs/regex/latest/regex/struct.Regex.html#method.splitn
  pub fn splitn(&self, text: &str, limit: usize) -> JsValue {
    let splits: Vec<String> = self
      .regex
      .splitn(text, limit)
      .map(|s| s.to_string())
      .collect();
    splits.to_js()
  }

  /// Returns the end location of a match in the text given.
  /// This method may have the same performance characteristics as `is_match`, except it provides an end location for a match. In particular, the location returned may be shorter than the proper end of the leftmost-first match.
  /// @see https://docs.rs/regex/latest/regex/struct.Regex.html#method.shortest_match
  #[wasm_bindgen(js_name = shortestMatch)]
  pub fn shortest_match(&self, text: &str) -> Option<usize> {
    self.regex.shortest_match(text)
  }

  /// Returns the same as shortest_match, but starts the search at the given offset.
  /// The significance of the starting point is that it takes the surrounding context into consideration. For example, the \A anchor can only match when `start == 0`.
  /// @see https://docs.rs/regex/latest/regex/struct.Regex.html#method.shortest_match_at
  #[wasm_bindgen(js_name = shortestMatchAt)]
  pub fn shortest_match_at(&self, text: &str, limit: usize) -> Option<usize> {
    if text.len() < limit {
      None
    } else {
      self.regex.shortest_match_at(text, limit)
    }
  }

  /// Return the Regex syntax object
  pub fn syntax(&self) -> JsValue {
    let mut parser = Parser::new();
    result(parser.parse(self.regex.as_str()))
  }

  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> String {
    self.regex.as_str().to_owned()
  }
}

#[wasm_bindgen(typescript_custom_section)]
const HIR_TYPE: &'static str = r#"
export type Hir = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Hir'
  bold: boolean;
  italic: boolean;
  size: number;
}
"#;

impl ToJs for hir::Hir {
  fn to_js(&self) -> JsValue {
    JsObject!("@type" => "struct", "@name" => "regex_syntax::hir::Hir", "kind" => self.kind().to_js())
  }
}


#[wasm_bindgen(typescript_custom_section)]
const HIRKIND_TYPE: &'static str = r#"
export type HirKind =
  | HirKindEmptyVariant
  | HirKindLiteralVariant
  | HirKindClassVariant
  | HirKindAnchorVariant
  | HirKindWordBoundaryVariant
  | HirKindRepetitionVariant
  | HirKindGroupVariant
  | HirKindConcatVariant
  | HirKindAlternationVariant

export type HirKindEmptyVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::HirKind'
  '@variant': 'Empty'
}

export type HirKindLiteralVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::HirKind'
  '@variant': 'Literal'
  'value': Literal
}

export type HirKindClassVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::HirKind'
  '@variant': 'Class'
  'value': Class
}

export type HirKindAnchorVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::HirKind'
  '@variant': 'Anchor'
  'value': Anchor
}

export type HirKindWordBoundaryVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::HirKind'
  '@variant': 'WordBoundary'
  'value': WordBoundary
}

export type HirKindRepetitionVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::HirKind'
  '@variant': 'Repetition'
  'value': Repetition
}

export type HirKindGroupVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::HirKind'
  '@variant': 'Group'
  'value': Group
}

export type HirKindConcatVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::HirKind'
  '@variant': 'Concat'
  'value': Hir[]
}

export type HirKindAlternationVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::HirKind'
  '@variant': 'Alternation'
  'value': Hir[]
}
"#;

impl ToJs for hir::HirKind {
  fn to_js(&self) -> JsValue {
    let current = JsObject!("@type" => "enum", "@name" => "regex_syntax::hir::HirKind");
    match self {
      hir::HirKind::Empty => set!(&current, "@variant" => "Empty"),
      hir::HirKind::Literal(l) => set!(&current, "@variant" => "Literal", "value" => l.to_js()),
      hir::HirKind::Class(c) => set!(&current, "@variant" => "Class", "value" => c.to_js()),
      hir::HirKind::Anchor(a) => set!(&current, "@variant" => "Anchor", "value" => a.to_js()),
      hir::HirKind::WordBoundary(w) => set!(&current, "@variant" => "WordBoundary", "value" => w.to_js()),
      hir::HirKind::Repetition(r) => set!(&current, "@variant" => "Repetition", "value" => r.to_js()),
      hir::HirKind::Group(g) => set!(&current, "@variant" => "Group", "value" => g.to_js()),
      hir::HirKind::Concat(c) => set!(&current, "@variant" => "Concat", "value" => c.to_js()),
      hir::HirKind::Alternation(c) => set!(&current, "@variant" => "Alternation", "value" => c.to_js()),
    };
    current
  }
}

#[wasm_bindgen(typescript_custom_section)]
const LITERAL_TYPE: &'static str = r#"
export type Literal =
  | LiteralUnicodeVariant
  | LiteralByteVariant

export type LiteralUnicodeVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Literal'
  '@variant': 'Unicode'
  'value': string
}

export type LiteralByteVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Literal'
  '@variant': 'Byte'
  'value': number
}
"#;

impl ToJs for hir::Literal {
  fn to_js(&self) -> JsValue {
    let current = JsObject!("@type" => "enum", "@name" => "regex_syntax::hir::Literal");
    match self {
      hir::Literal::Unicode(unicode) => set!(&current, "@variant" => "Unicode", "value" => unicode.to_string()),
      hir::Literal::Byte(byte) => set!(&current,  "@variant" => "Byte", "value" => byte.to_owned() as i32),
    };
    current
  }
}

#[wasm_bindgen(typescript_custom_section)]
const CLASS_TYPE: &'static str = r#"
export type Class =
  | ClassUnicodeVariant
  | ClassByteVariant

export type ClassUnicodeVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Class'
  '@variant': 'Unicode'
  'value': ClassUnicode
}

export type ClassByteVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Class'
  '@variant': 'Bytes'
  'value': ClassBytes
}
"#;

impl ToJs for hir::Class {
  fn to_js(&self) -> JsValue {
    let current = JsObject!("@type" => "enum", "@name" => "regex_syntax::hir::Class");
    match self {
      hir::Class::Unicode(unicode) => set!(&current, "@variant" => "Unicode", "value" => unicode.to_js()),
      hir::Class::Bytes(byte) => set!(&current, "@variant" => "Bytes", "value" => byte.to_js()),
    };
    current
  }
}

#[wasm_bindgen(typescript_custom_section)]
const CLASSUNICODE_TYPE: &'static str = r#"
export type ClassUnicode = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::ClassUnicode'
  'ranges': ClassUnicodeRange[]
}
"#;

impl ToJs for hir::ClassUnicode {
  fn to_js(&self) -> JsValue {
    let current = JsObject!("@type" => "struct", "@name" => "regex_syntax::hir::ClassUnicode");
    let ranges: Vec<hir::ClassUnicodeRange> = self.ranges().iter().map(|c| c.to_owned()).collect();
    set!(&current, "ranges" => ranges.to_js());
    current
  }
}

#[wasm_bindgen(typescript_custom_section)]
const CLASSUNICODERANGE_TYPE: &'static str = r#"
export type ClassUnicodeRange = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::ClassUnicodeRange'
  start: string
  end: string
}
"#;

impl ToJs for hir::ClassUnicodeRange {
  fn to_js(&self) -> JsValue {
    JsObject!(
      "@type" => "struct",
      "@name" => "regex_syntax::hir::ClassUnicodeRange",
      "start" => self.start().to_string(),
      "end" => self.end().to_string()
    )
  }
}

#[wasm_bindgen(typescript_custom_section)]
const CLASSBYTES_TYPE: &'static str = r#"
export type ClassBytes = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::ClassBytes'
  ranges: ClassBytesRange[]
}
"#;

impl ToJs for hir::ClassBytes {
  fn to_js(&self) -> JsValue {
    let current = JsObject!("@type" => "struct", "@name" => "regex_syntax::hir::ClassBytes");
    let ranges: Vec<hir::ClassBytesRange> = self.ranges().iter().map(|c| c.to_owned()).collect();
    set!(&current, "ranges" => ranges.to_js());
    current
  }
}

#[wasm_bindgen(typescript_custom_section)]
const CLASSBYTESRANGE_TYPE: &'static str = r#"
export type ClassBytesRange = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::ClassBytesRange'
  start: number
  end: number
}
"#;

impl ToJs for hir::ClassBytesRange {
  fn to_js(&self) -> JsValue {
    JsObject!(
      "@type" => "struct",
      "@name" => "regex_syntax::hir::ClassBytesRange",
      "start" => self.start().to_owned() as i32,
      "end" => self.end().to_owned() as i32
    )
  }
}

#[wasm_bindgen(typescript_custom_section)]
const ANCHOR_TYPE: &'static str = r#"
export type Anchor =
  | AnchorStartLineVariant
  | AnchorEndLineVariant
  | AnchorStartTextVariant
  | AnchorEndTextVariant

export type AnchorStartLineVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Anchor'
  '@variant': 'StartLine'
}

export type AnchorEndLineVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Anchor'
  '@variant': 'EndLine'
}

export type AnchorStartTextVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Anchor'
  '@variant': 'StartText'
}

export type AnchorEndTextVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Anchor'
  '@variant': 'EndText'
}
"#;

impl ToJs for hir::Anchor {
  fn to_js(&self) -> JsValue {
    let current = JsObject!("@type" => "enum", "@name" => "regex_syntax::hir::Anchor");
    match self {
      hir::Anchor::StartLine => set!(&current, "@variant" => "StartLine"),
      hir::Anchor::EndLine => set!(&current, "@variant" => "EndLine"),
      hir::Anchor::StartText => set!(&current, "@variant" => "StartText"),
      hir::Anchor::EndText => set!(&current, "@variant" => "EndText"),
    };
    current
  }
}

#[wasm_bindgen(typescript_custom_section)]
const WORDBOUNDARY_TYPE: &'static str = r#"
export type WordBoundary =
  | WordBoundaryAsciiVariant
  | WordBoundaryAsciiNegateVariant
  | WordBoundaryUnicodeVariant
  | WordBoundaryUnicodeNegateVariant

export type WordBoundaryAsciiVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::WordBoundary'
  '@variant': 'Ascii'
}

export type WordBoundaryAsciiNegateVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::WordBoundary'
  '@variant': 'AsciiNegate'
}

export type WordBoundaryUnicodeVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::WordBoundary'
  '@variant': 'Unicode'
}

export type WordBoundaryUnicodeNegateVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::WordBoundary'
  '@variant': 'UnicodeNegate'
}
"#;

impl ToJs for hir::WordBoundary {
  fn to_js(&self) -> JsValue {
    let current = JsObject!("@type" => "enum", "@name" => "regex_syntax::hir::WordBoundary");
    match self {
      hir::WordBoundary::Ascii => set!(&current, "@variant" => "Ascii"),
      hir::WordBoundary::AsciiNegate => set!(&current, "@variant" => "AsciiNegate"),
      hir::WordBoundary::Unicode => set!(&current, "@variant" => "Unicode"),
      hir::WordBoundary::UnicodeNegate => set!(&current, "@variant" => "UnicodeNegate"),
    };
    current
  }
}

#[wasm_bindgen(typescript_custom_section)]
const REPETITION_TYPE: &'static str = r#"
export type Repetition = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Repetition'
  greedy: boolean
  kind: RepetitionKind
  hir: Hir
}
"#;

impl ToJs for hir::Repetition {
  fn to_js(&self) -> JsValue {
    JsObject!(
      "@type" => "struct",
      "@name" => "regex_syntax::hir::Repetition",
      "greedy" => self.greedy,
      "kind" => self.kind.to_js(),
      "hir" => self.hir.to_js()
    )
  }
}

#[wasm_bindgen(typescript_custom_section)]
const REPETITIONKIND_TYPE: &'static str = r#"
export type RepetitionKind =
  | RepetitionKindZeroOrOneVariant
  | RepetitionKindZeroOrMoreVariant
  | RepetitionKindOneOrMoreVariant
  | RepetitionKindRangeVariant

export type RepetitionKindZeroOrOneVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Repetition'
  '@variant': 'ZeroOrOne'
}

export type RepetitionKindZeroOrMoreVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Repetition'
  '@variant': 'ZeroOrMore'
}

export type RepetitionKindOneOrMoreVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Repetition'
  '@variant': 'OneOrMore'
}

export type RepetitionKindRangeVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Repetition'
  '@variant': 'Range'
  value: RepetitionRange
}
"#;

impl ToJs for hir::RepetitionKind {
  fn to_js(&self) -> JsValue {
    let current = JsObject!("@type" => "enum", "@name" => "regex_syntax::hir::RepetitionKind");
    match self {
      hir::RepetitionKind::ZeroOrOne => set!(&current, "@variant" => "ZeroOrOne"),
      hir::RepetitionKind::ZeroOrMore => set!(&current, "@variant" => "ZeroOrMore"),
      hir::RepetitionKind::OneOrMore => set!(&current, "@variant" => "OneOrMore"),
      hir::RepetitionKind::Range(range) => set!(&current, "@variant" => "Range", "value" => range.to_js()),
    };
    current
  }
}

#[wasm_bindgen(typescript_custom_section)]
const REPETITIONRANGE_TYPE: &'static str = r#"
export type RepetitionRange =
  | RepetitionRangeExactlyVariant
  | RepetitionRangeAtLeastVariant
  | RepetitionRangeBoundedVariant

export type RepetitionRangeExactlyVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::RepetitionRange'
  '@variant': 'Exactly'
  value: number
}

export type RepetitionRangeAtLeastVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::RepetitionRange'
  '@variant': 'AtLeast'
  value: number
}

export type RepetitionRangeBoundedVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::RepetitionRange'
  '@variant': 'Bounded'
  value: [number, number]
}
"#;

impl ToJs for hir::RepetitionRange {
  fn to_js(&self) -> JsValue {
    let current = JsObject!("@type" => "enum", "@name" => "regex_syntax::hir::RepetitionRange");
    match self {
      hir::RepetitionRange::Exactly(val) => set!(&current, "@variant" => "Exactly", "value" => val.to_owned() as f64),
      hir::RepetitionRange::AtLeast(min) => set!(&current, "@variant" => "AtLeast", "value" => min.to_owned() as f64),
      hir::RepetitionRange::Bounded(min, max) => set!(&current, "@variant" => "Bounded", "value" => JsArray!(min.to_owned() as f64, max.to_owned() as f64)),
    };
    current
  }
}

#[wasm_bindgen(typescript_custom_section)]
const GROUP_TYPE: &'static str = r#"
export type Group = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::Group'
  kind: GroupKind
  hir: Hir
}
"#;

impl ToJs for hir::Group {
  fn to_js(&self) -> JsValue {
    JsObject!(
      "@type" => "struct",
      "@name" => "regex_syntax::hir::Group",
      "kind" => self.kind.to_js(),
      "hir" => self.hir.to_js()
    )
  }
}

#[wasm_bindgen(typescript_custom_section)]
const GROUPKIND_TYPE: &'static str = r#"
export type GroupKind =
  | GroupKindCaptureIndexVariant
  | GroupKindCaptureNameVariant
  | GroupKindNonCapturingVariant

export type GroupKindCaptureIndexVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::GroupKind'
  '@variant': 'CaptureIndex'
  index: number
}

export type GroupKindCaptureNameVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::GroupKind'
  '@variant': 'CaptureName'
  index: number
  name: string
}

export type GroupKindNonCapturingVariant = {
  '@type': 'struct'
  '@name': 'regex_syntax::hir::GroupKind'
  '@variant': 'NonCapturing'
}
"#;

impl ToJs for hir::GroupKind {
  fn to_js(&self) -> JsValue {
    let current = JsObject!("@type" => "enum", "@name" => "regex_syntax::hir::GroupKind");
    match self {
      hir::GroupKind::CaptureIndex(index) => set!(&current, "@variant" => "CaptureIndex", "index" => index.to_owned() as f64),
      hir::GroupKind::CaptureName { name, index } => set!(&current, "@variant" => "CaptureName", "index" => index.to_owned() as f64, "name" => name.to_owned()),
      hir::GroupKind::NonCapturing => set!(&current, "@variant" => "NonCapturing"),
    };
    current
  }
}

#[wasm_bindgen(typescript_custom_section)]
const MATCH_TYPE: &'static str = r#"
export type Match = {
  start: number
  end: number
  value: string
}
"#;

impl<'t> ToJs for regex::Match<'t> {
  fn to_js(&self) -> JsValue {
    JsObject!(
      "start" => self.start() as f64,
      "end" => self.end() as f64,
      "value" => self.as_str().to_owned().to_js()
    )
  }
}
