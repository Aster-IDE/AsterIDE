//! Plugin system for `AsterIDE`.
//!
//! Hosts the hikari (光) syntax-highlighting registry — the batteries-included
//! `Ecosystem` of language plugins. This is the M0 seam: the editor resolves a
//! file path to a `Box<dyn Highlighter>` through here. See
//! `pleme-io/theory/HIKARI.md` for the full pluggable-ecosystem design (M1
//! extracts hikari to its own fleet repo and adds tree-sitter + `(deflexer)`
//! backends; the trait surface consumed here does not change).

pub use hikari_core::{
    ByteSpan, Ecosystem, HighlightSpan, Highlighter, HlClass, Language, NordTheme, Rgb, Theme,
};

/// The `AsterIDE` language registry: the batteries-included hikari `Ecosystem`
/// plus the default (Nord) theme.
pub struct LanguageRegistry {
    ecosystem: Ecosystem,
    theme: NordTheme,
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ecosystem: Ecosystem::with_builtins(),
            theme: NordTheme,
        }
    }

    /// The set of languages the editor can currently highlight.
    #[must_use]
    pub fn languages(&self) -> Vec<Language> {
        self.ecosystem.languages()
    }

    /// Resolve a file path (or name) to its highlighter — plain text if the
    /// language is unknown (never a panic, never "everything is Rust").
    #[must_use]
    pub fn highlighter_for_path(&self, path: &str) -> Box<dyn Highlighter> {
        self.ecosystem.highlighter_for_path(path)
    }

    /// The active theme's color for a highlight class.
    #[must_use]
    pub fn color(&self, class: HlClass) -> Rgb {
        self.theme.color(class)
    }
}
