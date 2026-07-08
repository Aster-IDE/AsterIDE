//! hikari (光) — the pluggable syntax-highlighting spine.
//!
//! This is the M0 vendored proving ground inside AsterIDE. The destination is
//! the fleet-shared `pleme-io/hikari` subsystem (see
//! `pleme-io/theory/HIKARI.md`); M1 extracts this crate unchanged and adds the
//! tree-sitter backend (over escriba-ts) + the macro-generated `(deflexer …)`
//! backend, then redistributes to escriba and the rest of the fleet.
//!
//! The design (frozen at M0 so no later phase is a rewrite):
//!
//!   * ONE narrow-waist output — a coverage-complete, non-overlapping,
//!     forward-only `Vec<HighlightSpan>` partition produced ONLY through
//!     [`SpanSink`], which makes gaps / overlaps / reversals structurally
//!     unrepresentable (the caller cannot fabricate the `Vec`).
//!   * ONE authored backend trait [`LanguageLexer`] whose associated
//!     `LineState: Copy + Eq` makes incremental line-restart re-lex a *type
//!     property* (M1 uses it; M0 re-lexes the whole buffer).
//!   * ONE object-safe [`Highlighter`] the render layer holds as `Box<dyn>`,
//!     bridged from any `LanguageLexer` by the blanket [`LineDriven`] adapter.
//!   * ONE [`Ecosystem`] registry with total, panic-free resolution — an
//!     unknown extension is [`PLAIN_TEXT`], never a panic, never "unknown
//!     language". Fixes AsterIDE's every-file-highlighted-as-Rust bug.
//!   * palette-independent [`HlClass`] + a [`Theme`] mapping class → [`Rgb`]
//!     so classification and color never entangle (Nord default).

#![forbid(unsafe_code)]

// ───────────────────────────── span ─────────────────────────────

/// A byte-offset span into the document. `start <= end` by construction, and
/// (when produced through [`SpanSink`]) both ends land on UTF-8 char
/// boundaries.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ByteSpan {
    pub start: u32,
    pub end: u32,
}

impl ByteSpan {
    /// The sole constructor. Panics only on a caller bug (`start > end`); the
    /// lexer driver never constructs a reversed span.
    #[must_use]
    pub fn new(start: u32, end: u32) -> Self {
        assert!(start <= end, "ByteSpan::new: start {start} > end {end}");
        Self { start, end }
    }

    #[must_use]
    pub fn len(&self) -> u32 {
        self.end - self.start
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// The span as a `usize` range, for slicing the source text.
    #[must_use]
    pub fn range(&self) -> std::ops::Range<usize> {
        self.start as usize..self.end as usize
    }
}

// ───────────────────────────── class ────────────────────────────

/// A palette-independent semantic highlight class. A superset of the classes
/// egui / tree-sitter / the fleet's `Semantic` enums produce, so every backend
/// lowers to this one waist and the theme layer maps it to color exactly once.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum HlClass {
    Comment { multiline: bool },
    Keyword,
    /// A `:keyword`-style argument / symbol (lisp `:kw`, etc.).
    KeywordArg,
    Type,
    Function,
    Namespace,
    Variable,
    Constant,
    Str,
    Escape,
    Numeric { float: bool },
    Boolean,
    Punctuation,
    Operator,
    Attribute,
    Special,
    Hyperlink,
    Whitespace,
    Error,
    /// The default / unclassified class. Coverage gaps are filled with this.
    Plain,
}

/// One classified region of the document.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HighlightSpan {
    pub span: ByteSpan,
    pub class: HlClass,
}

// ───────────────────────────── sink ─────────────────────────────

/// The ONLY way to produce highlight spans. A forward-only cursor over a
/// single line: [`push`](SpanSink::push) fills any gap before the pushed
/// region with [`HlClass::Plain`] and clamps backwards writes, so the emitted
/// sequence is a coverage-complete, non-overlapping, monotonically-increasing
/// partition **by construction** — a lexer cannot emit a gap, an overlap, or a
/// reversal. The backing `Vec` is private; only the driver calls
/// [`finish`](SpanSink::finish).
pub struct SpanSink {
    cursor: u32,
    line_end: u32,
    out: Vec<HighlightSpan>,
}

impl SpanSink {
    fn new(line_start: u32, line_len: u32) -> Self {
        Self {
            cursor: line_start,
            line_end: line_start + line_len,
            out: Vec::new(),
        }
    }

    /// Classify `[start, end)` (absolute byte offsets) as `class`. A gap
    /// `[cursor, start)` is filled with [`HlClass::Plain`]; a backwards
    /// `start` is clamped to the cursor; an empty region is dropped.
    pub fn push(&mut self, start: u32, end: u32, class: HlClass) {
        let start = start.max(self.cursor);
        let end = end.min(self.line_end);
        if end <= start {
            return;
        }
        if start > self.cursor {
            self.out.push(HighlightSpan {
                span: ByteSpan::new(self.cursor, start),
                class: HlClass::Plain,
            });
        }
        self.out.push(HighlightSpan {
            span: ByteSpan::new(start, end),
            class,
        });
        self.cursor = end;
    }

    fn finish(mut self) -> Vec<HighlightSpan> {
        if self.cursor < self.line_end {
            self.out.push(HighlightSpan {
                span: ByteSpan::new(self.cursor, self.line_end),
                class: HlClass::Plain,
            });
        }
        self.out
    }
}

// ─────────────────────── backend + render traits ────────────────

/// The AUTHORED backend trait — implement this to add a language. One total
/// method lexes a single line given the cross-line state carried out of the
/// previous line; returning the state at the line's end. `LineState: Eq` is
/// what makes M1's incremental re-lex a type property (re-lex stops at the
/// first line whose entry state is unchanged).
pub trait LanguageLexer: Send + Sync {
    type LineState: Copy + Eq + Default + Send + Sync;

    fn lex_line(
        &self,
        line: &str,
        line_start: u32,
        entry: Self::LineState,
        sink: &mut SpanSink,
    ) -> Self::LineState;
}

/// The object-safe, render-facing trait the editor holds as `Box<dyn>`. M0
/// re-lexes the whole document per call (non-incremental but through the final
/// interface); M1 adds a line cache + the `LineState` early-stop.
pub trait Highlighter: Send + Sync {
    fn highlight(&self, text: &str) -> Vec<HighlightSpan>;
}

/// The blanket bridge: any [`LanguageLexer`] is a [`Highlighter`]. No backend
/// author ever hand-writes `Highlighter`.
pub struct LineDriven<L: LanguageLexer> {
    pub lexer: L,
}

impl<L: LanguageLexer> LineDriven<L> {
    #[must_use]
    pub fn new(lexer: L) -> Self {
        Self { lexer }
    }
}

impl<L: LanguageLexer> Highlighter for LineDriven<L> {
    fn highlight(&self, text: &str) -> Vec<HighlightSpan> {
        let mut out = Vec::new();
        let mut state = L::LineState::default();
        let mut offset: u32 = 0;
        // split_inclusive keeps the trailing '\n' on each line, so offsets are
        // contiguous and the partition covers every byte of `text`.
        for line in text.split_inclusive('\n') {
            let line_len = u32::try_from(line.len()).unwrap_or(u32::MAX);
            let mut sink = SpanSink::new(offset, line_len);
            state = self.lexer.lex_line(line, offset, state, &mut sink);
            out.extend(sink.finish());
            offset = offset.saturating_add(line_len);
        }
        out
    }
}

// ───────────────────────── plugin + registry ────────────────────

/// A language identity — an interned static name (closed by linkage, no open
/// enum to keep exhaustive).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Language(pub &'static str);

/// The unclassified fallback language — its highlighter emits everything as
/// [`HlClass::Plain`].
pub const PLAIN_TEXT: Language = Language("plaintext");

/// How a plugin claims a document.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Selector {
    /// A file extension without the dot, lowercase (e.g. `"rs"`).
    Extension(&'static str),
    /// An exact file name (e.g. `"Cargo.toml"`).
    Filename(&'static str),
}

/// A pluggable language backend: identity + how it's selected + its factory.
/// Satisfied identically by the M0 table lexers and (M1+) tree-sitter / wasm.
pub trait LanguagePlugin: Send + Sync {
    fn language(&self) -> Language;
    fn selectors(&self) -> &'static [Selector];
    fn make_highlighter(&self) -> Box<dyn Highlighter>;
}

/// The registry. Total resolution: an unmatched path resolves to
/// [`PLAIN_TEXT`], never a panic.
pub struct Ecosystem {
    plugins: Vec<Box<dyn LanguagePlugin>>,
}

impl Default for Ecosystem {
    fn default() -> Self {
        Self::with_builtins()
    }
}

impl Ecosystem {
    /// An empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// The batteries-included default: every built-in language plugin.
    #[must_use]
    pub fn with_builtins() -> Self {
        let mut eco = Self::new();
        for p in langs::builtins() {
            eco.plugins.push(p);
        }
        eco
    }

    pub fn register(&mut self, plugin: Box<dyn LanguagePlugin>) {
        self.plugins.push(plugin);
    }

    /// All languages the registry can highlight.
    #[must_use]
    pub fn languages(&self) -> Vec<Language> {
        self.plugins.iter().map(|p| p.language()).collect()
    }

    /// Resolve a file path to a language. Filename match wins over extension;
    /// no match resolves to [`PLAIN_TEXT`].
    #[must_use]
    pub fn resolve(&self, path: &str) -> Language {
        let name = path.rsplit(['/', '\\']).next().unwrap_or(path);
        for p in &self.plugins {
            for sel in p.selectors() {
                if let Selector::Filename(f) = sel {
                    if name.eq_ignore_ascii_case(f) {
                        return p.language();
                    }
                }
            }
        }
        if let Some(ext) = name.rsplit_once('.').map(|(_, e)| e) {
            for p in &self.plugins {
                for sel in p.selectors() {
                    if let Selector::Extension(e) = sel {
                        if ext.eq_ignore_ascii_case(e) {
                            return p.language();
                        }
                    }
                }
            }
        }
        PLAIN_TEXT
    }

    /// A highlighter for a language, or the plain-text highlighter if none is
    /// registered for it.
    #[must_use]
    pub fn highlighter_for(&self, lang: Language) -> Box<dyn Highlighter> {
        for p in &self.plugins {
            if p.language() == lang {
                return p.make_highlighter();
            }
        }
        Box::new(PlainHighlighter)
    }

    /// The one call an editor needs: path → highlighter.
    #[must_use]
    pub fn highlighter_for_path(&self, path: &str) -> Box<dyn Highlighter> {
        self.highlighter_for(self.resolve(path))
    }
}

/// The plain-text highlighter: one `Plain` span over the whole text.
pub struct PlainHighlighter;

impl Highlighter for PlainHighlighter {
    fn highlight(&self, text: &str) -> Vec<HighlightSpan> {
        if text.is_empty() {
            return Vec::new();
        }
        vec![HighlightSpan {
            span: ByteSpan::new(0, u32::try_from(text.len()).unwrap_or(u32::MAX)),
            class: HlClass::Plain,
        }]
    }
}

// ───────────────────────────── theme ────────────────────────────

/// An 8-bit sRGB color. Palette-independent; the editor maps it to its own
/// color type (egui `Color32`, ratatui `Color`, …).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// Maps a [`HlClass`] to a color. The default is Nord (the fleet look — see
/// mado's default). M1 sources this from `ishou_tokens` instead of literals.
pub trait Theme: Send + Sync {
    fn color(&self, class: HlClass) -> Rgb;
}

/// The Nord Polar Night / Snow Storm / Aurora / Frost palette.
pub struct NordTheme;

impl Theme for NordTheme {
    fn color(&self, class: HlClass) -> Rgb {
        // Nord anchors: fg snow #D8DEE9, comment polar #616E88, keyword frost
        // #81A1C1, string aurora-green #A3BE8C, number aurora-purple #B48EAD,
        // type frost #8FBCBB, function frost #88C0D0, constant/bool orange
        // #D08770, punctuation snow #ECEFF4, error red #BF616A.
        match class {
            HlClass::Comment { .. } => Rgb::new(0x61, 0x6E, 0x88),
            HlClass::Keyword => Rgb::new(0x81, 0xA1, 0xC1),
            HlClass::KeywordArg | HlClass::Attribute => Rgb::new(0xB4, 0x8E, 0xAD),
            HlClass::Type | HlClass::Namespace => Rgb::new(0x8F, 0xBC, 0xBB),
            HlClass::Function => Rgb::new(0x88, 0xC0, 0xD0),
            HlClass::Str => Rgb::new(0xA3, 0xBE, 0x8C),
            HlClass::Escape | HlClass::Special => Rgb::new(0xEB, 0xCB, 0x8B),
            HlClass::Numeric { .. } => Rgb::new(0xB4, 0x8E, 0xAD),
            HlClass::Boolean | HlClass::Constant => Rgb::new(0xD0, 0x87, 0x70),
            HlClass::Operator => Rgb::new(0x81, 0xA1, 0xC1),
            HlClass::Punctuation => Rgb::new(0xEC, 0xEF, 0xF4),
            HlClass::Hyperlink => Rgb::new(0x5E, 0x81, 0xAC),
            HlClass::Error => Rgb::new(0xBF, 0x61, 0x6A),
            HlClass::Variable | HlClass::Whitespace | HlClass::Plain => Rgb::new(0xD8, 0xDE, 0xE9),
        }
    }
}

// ─────────────────────── the table-driven backend ───────────────

/// The hand-rolled backend (M0). One byte/char-class scanner parameterized by
/// a per-language [`LangTable`], covering the common shape of the go-wide
/// languages. M2 replaces the table with the macro-generated `(deflexer …)`
/// output; the trait it satisfies is unchanged.
pub mod langs {
    use super::{
        HlClass, Language, LanguageLexer, LanguagePlugin, LineDriven, Selector, SpanSink,
    };

    /// Per-language lexing table.
    pub struct LangTable {
        pub keywords: &'static [&'static str],
        pub line_comments: &'static [&'static str],
        pub block_comment: Option<(&'static str, &'static str)>,
        pub string_delims: &'static [char],
        /// Treat `:name` tokens as [`HlClass::KeywordArg`] (lisp keywords).
        pub colon_keywords: bool,
    }

    /// Cross-line state: continuing a block comment or a string literal.
    #[derive(Clone, Copy, PartialEq, Eq, Default)]
    pub enum LineMode {
        #[default]
        Normal,
        InBlockComment,
        /// Inside a string opened on a previous line; carries the delimiter.
        InString(char),
    }

    /// The table lexer.
    pub struct TableLexer {
        pub table: &'static LangTable,
    }

    #[inline]
    fn is_ident_start(c: char) -> bool {
        c == '_' || c.is_alphabetic()
    }
    #[inline]
    fn is_ident_continue(c: char) -> bool {
        c == '_' || c.is_alphanumeric()
    }

    impl LanguageLexer for TableLexer {
        type LineState = LineMode;

        #[allow(clippy::too_many_lines)]
        fn lex_line(
            &self,
            line: &str,
            line_start: u32,
            entry: LineMode,
            sink: &mut SpanSink,
        ) -> LineMode {
            let t = self.table;
            let n = line.len();
            let base = line_start;
            let push = |sink: &mut SpanSink, s: usize, e: usize, class: HlClass| {
                sink.push(base + s as u32, base + e as u32, class);
            };
            let mut i = 0usize;
            let mut mode = entry;

            // Resume a continuation from the previous line.
            match mode {
                LineMode::InBlockComment => {
                    if let Some((_, close)) = t.block_comment {
                        if let Some(rel) = line.find(close) {
                            let e = rel + close.len();
                            push(sink, 0, e, HlClass::Comment { multiline: true });
                            i = e;
                            mode = LineMode::Normal;
                        } else {
                            push(sink, 0, n, HlClass::Comment { multiline: true });
                            return LineMode::InBlockComment;
                        }
                    } else {
                        mode = LineMode::Normal;
                    }
                }
                LineMode::InString(delim) => {
                    let e = scan_string_body(line, 0, delim);
                    match e {
                        Some(end) => {
                            push(sink, 0, end, HlClass::Str);
                            i = end;
                            mode = LineMode::Normal;
                        }
                        None => {
                            push(sink, 0, n, HlClass::Str);
                            return LineMode::InString(delim);
                        }
                    }
                }
                LineMode::Normal => {}
            }

            let _ = mode;
            'scan: while i < n {
                let c = line[i..].chars().next().unwrap();
                let cl = c.len_utf8();

                // whitespace run
                if c.is_whitespace() {
                    let s = i;
                    while i < n {
                        let d = line[i..].chars().next().unwrap();
                        if !d.is_whitespace() {
                            break;
                        }
                        i += d.len_utf8();
                    }
                    push(sink, s, i, HlClass::Whitespace);
                    continue 'scan;
                }

                // line comments
                for lc in t.line_comments {
                    if line[i..].starts_with(lc) {
                        push(sink, i, n, HlClass::Comment { multiline: false });
                        i = n;
                        continue 'scan;
                    }
                }

                // block comment open
                if let Some((open, close)) = t.block_comment {
                    if line[i..].starts_with(open) {
                        if let Some(rel) = line[i + open.len()..].find(close) {
                            let e = i + open.len() + rel + close.len();
                            push(sink, i, e, HlClass::Comment { multiline: true });
                            i = e;
                            continue 'scan;
                        }
                        push(sink, i, n, HlClass::Comment { multiline: true });
                        return LineMode::InBlockComment;
                    }
                }

                // string literal
                if t.string_delims.contains(&c) {
                    match scan_string_body(line, i + cl, c) {
                        Some(end) => {
                            push(sink, i, end, HlClass::Str);
                            i = end;
                            continue 'scan;
                        }
                        None => {
                            push(sink, i, n, HlClass::Str);
                            return LineMode::InString(c);
                        }
                    }
                }

                // number
                if c.is_ascii_digit() {
                    let s = i;
                    let mut is_float = false;
                    i += cl;
                    while i < n {
                        let d = line[i..].chars().next().unwrap();
                        if d.is_ascii_alphanumeric() || d == '_' {
                            i += d.len_utf8();
                        } else if d == '.' {
                            is_float = true;
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    push(sink, s, i, HlClass::Numeric { float: is_float });
                    continue 'scan;
                }

                // colon keyword (lisp `:name`)
                if t.colon_keywords && c == ':' && i + 1 < n {
                    let next = line[i + 1..].chars().next().unwrap();
                    if is_ident_start(next) {
                        let s = i;
                        i += 1;
                        while i < n {
                            let d = line[i..].chars().next().unwrap();
                            if !is_ident_continue(d) {
                                break;
                            }
                            i += d.len_utf8();
                        }
                        push(sink, s, i, HlClass::KeywordArg);
                        continue 'scan;
                    }
                }

                // identifier / keyword
                if is_ident_start(c) {
                    let s = i;
                    i += cl;
                    while i < n {
                        let d = line[i..].chars().next().unwrap();
                        if !is_ident_continue(d) {
                            break;
                        }
                        i += d.len_utf8();
                    }
                    let word = &line[s..i];
                    let class = if t.keywords.contains(&word) {
                        HlClass::Keyword
                    } else if matches!(word, "true" | "false" | "True" | "False" | "None" | "nil" | "null") {
                        HlClass::Boolean
                    } else if word.chars().next().is_some_and(char::is_uppercase) {
                        HlClass::Type
                    } else {
                        HlClass::Variable
                    };
                    push(sink, s, i, class);
                    continue 'scan;
                }

                // punctuation / operator (single char)
                let class = if "+-*/%=<>!&|^~".contains(c) {
                    HlClass::Operator
                } else {
                    HlClass::Punctuation
                };
                push(sink, i, i + cl, class);
                i += cl;
            }

            LineMode::Normal
        }
    }

    /// Scan a string body starting at `from` (just after the opening delim),
    /// honoring `\` escapes. Returns the byte index just past the closing
    /// delim, or `None` if the line ends first (string continues).
    fn scan_string_body(line: &str, from: usize, delim: char) -> Option<usize> {
        let n = line.len();
        let mut i = from;
        while i < n {
            let c = line[i..].chars().next().unwrap();
            let cl = c.len_utf8();
            if c == '\\' && i + cl < n {
                let e = line[i + cl..].chars().next().unwrap();
                i += cl + e.len_utf8();
                continue;
            }
            i += cl;
            if c == delim {
                return Some(i);
            }
        }
        None
    }

    /// A [`LanguagePlugin`] backed by a static [`LangTable`].
    pub struct TablePlugin {
        pub language: Language,
        pub selectors: &'static [Selector],
        pub table: &'static LangTable,
    }

    impl LanguagePlugin for TablePlugin {
        fn language(&self) -> Language {
            self.language
        }
        fn selectors(&self) -> &'static [Selector] {
            self.selectors
        }
        fn make_highlighter(&self) -> Box<dyn super::Highlighter> {
            Box::new(LineDriven::new(TableLexer { table: self.table }))
        }
    }

    // ── the built-in language tables ──

    static RUST_KW: &[&str] = &[
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
        "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "type",
        "unsafe", "use", "where", "while",
    ];
    static RUST_TABLE: LangTable = LangTable {
        keywords: RUST_KW,
        line_comments: &["//"],
        block_comment: Some(("/*", "*/")),
        string_delims: &['"'],
        colon_keywords: false,
    };
    static RUST_SEL: &[Selector] = &[Selector::Extension("rs")];

    static PY_KW: &[&str] = &[
        "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del",
        "elif", "else", "except", "finally", "for", "from", "global", "if", "import", "in", "is",
        "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while", "with",
        "yield",
    ];
    static PY_TABLE: LangTable = LangTable {
        keywords: PY_KW,
        line_comments: &["#"],
        block_comment: None,
        string_delims: &['"', '\''],
        colon_keywords: false,
    };
    static PY_SEL: &[Selector] = &[Selector::Extension("py")];

    static LISP_KW: &[&str] = &[
        "def", "defn", "defmacro", "defcaixa", "deflexer", "let", "lambda", "fn", "if", "cond",
        "when", "unless", "do", "quote",
    ];
    static LISP_TABLE: LangTable = LangTable {
        keywords: LISP_KW,
        line_comments: &[";"],
        block_comment: Some(("#|", "|#")),
        string_delims: &['"'],
        colon_keywords: true,
    };
    static LISP_SEL: &[Selector] = &[
        Selector::Extension("lisp"),
        Selector::Extension("lsp"),
        Selector::Extension("el"),
        Selector::Extension("scm"),
    ];

    static JSON_TABLE: LangTable = LangTable {
        keywords: &["true", "false", "null"],
        line_comments: &[],
        block_comment: None,
        string_delims: &['"'],
        colon_keywords: false,
    };
    static JSON_SEL: &[Selector] = &[Selector::Extension("json")];

    static TOML_TABLE: LangTable = LangTable {
        keywords: &["true", "false"],
        line_comments: &["#"],
        block_comment: None,
        string_delims: &['"', '\''],
        colon_keywords: false,
    };
    static TOML_SEL: &[Selector] = &[
        Selector::Extension("toml"),
        Selector::Filename("Cargo.lock"),
    ];

    static MD_TABLE: LangTable = LangTable {
        keywords: &[],
        line_comments: &[],
        block_comment: None,
        string_delims: &['`'],
        colon_keywords: false,
    };
    static MD_SEL: &[Selector] = &[
        Selector::Extension("md"),
        Selector::Extension("markdown"),
    ];

    /// Every built-in language plugin (the batteries-included default set).
    #[must_use]
    pub fn builtins() -> Vec<Box<dyn LanguagePlugin>> {
        vec![
            Box::new(TablePlugin {
                language: Language("rust"),
                selectors: RUST_SEL,
                table: &RUST_TABLE,
            }),
            Box::new(TablePlugin {
                language: Language("python"),
                selectors: PY_SEL,
                table: &PY_TABLE,
            }),
            Box::new(TablePlugin {
                language: Language("lisp"),
                selectors: LISP_SEL,
                table: &LISP_TABLE,
            }),
            Box::new(TablePlugin {
                language: Language("json"),
                selectors: JSON_SEL,
                table: &JSON_TABLE,
            }),
            Box::new(TablePlugin {
                language: Language("toml"),
                selectors: TOML_SEL,
                table: &TOML_TABLE,
            }),
            Box::new(TablePlugin {
                language: Language("markdown"),
                selectors: MD_SEL,
                table: &MD_TABLE,
            }),
        ]
    }
}

// ───────────────────────────── tests ────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn covers(text: &str, spans: &[HighlightSpan]) {
        // Coverage-complete + non-overlapping + forward-only by construction.
        let mut cursor = 0u32;
        for s in spans {
            assert_eq!(s.span.start, cursor, "gap/overlap at {cursor}");
            assert!(s.span.end > s.span.start);
            cursor = s.span.end;
        }
        assert_eq!(cursor as usize, text.len(), "partition does not cover text");
    }

    #[test]
    fn partition_is_coverage_complete() {
        let eco = Ecosystem::with_builtins();
        for (path, src) in [
            ("a.rs", "fn main() {\n    let x = 42; // hi\n}\n"),
            ("b.py", "def f(x):\n    return \"s\"  # c\n"),
            ("c.lisp", "(defcaixa :name \"x\" 42) ; c\n"),
            ("d.txt", "no language here\n"),
        ] {
            let h = eco.highlighter_for_path(path);
            let spans = h.highlight(src);
            covers(src, &spans);
        }
    }

    #[test]
    fn resolves_by_extension_not_always_rust() {
        let eco = Ecosystem::with_builtins();
        assert_eq!(eco.resolve("src/main.rs"), Language("rust"));
        assert_eq!(eco.resolve("app.py"), Language("python"));
        assert_eq!(eco.resolve("x.lisp"), Language("lisp"));
        assert_eq!(eco.resolve("Cargo.lock"), Language("toml"));
        // The bug: a non-Rust file must NOT resolve to rust.
        assert_eq!(eco.resolve("notes.txt"), PLAIN_TEXT);
        assert_ne!(eco.resolve("app.py"), Language("rust"));
    }

    #[test]
    fn rust_keyword_is_classified() {
        let eco = Ecosystem::with_builtins();
        let spans = eco.highlighter_for_path("a.rs").highlight("fn x");
        assert_eq!(spans[0].class, HlClass::Keyword); // `fn`
    }

    #[test]
    fn multiline_string_and_block_comment_thread_state() {
        let eco = Ecosystem::with_builtins();
        // block comment spanning two lines
        let spans = eco.highlighter_for_path("a.rs").highlight("/* a\nb */ x\n");
        covers("/* a\nb */ x\n", &spans);
        assert!(matches!(spans[0].class, HlClass::Comment { multiline: true }));
    }

    #[test]
    fn plain_text_is_one_plain_span() {
        let h = PlainHighlighter;
        let spans = h.highlight("hello");
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].class, HlClass::Plain);
    }
}
