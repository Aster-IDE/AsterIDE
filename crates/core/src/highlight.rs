//! egui adapter for the hikari (光) syntax highlighter.
//!
//! Bridges hikari's palette-independent `Vec<HighlightSpan>` to an
//! `egui::text::LayoutJob` the code editor's `TextEdit` layouter consumes.
//! hikari-core itself stays egui-free (pure, fleet-portable); this thin
//! adapter — the only egui coupling — lives in the consumer per the M0 design.

use eframe::egui;
use plugins::{HlClass, Highlighter, NordTheme, Rgb, Theme};

fn color32(rgb: Rgb) -> egui::Color32 {
    egui::Color32::from_rgb(rgb.r, rgb.g, rgb.b)
}

/// Build a fully-colored `LayoutJob` for `text` using `highlighter` for
/// classification and the Nord palette for color. Monospace at `font_size`;
/// no wrapping (the editor scrolls horizontally). Coverage is guaranteed by
/// hikari's `SpanSink`, so every byte is colored exactly once.
#[must_use]
pub fn layout_job(text: &str, highlighter: &dyn Highlighter, font_size: f32) -> egui::text::LayoutJob {
    let theme = NordTheme;
    let font = egui::FontId::monospace(font_size);
    let mut job = egui::text::LayoutJob::default();
    job.wrap.max_width = f32::INFINITY;

    for hs in highlighter.highlight(text) {
        let piece = &text[hs.span.range()];
        job.append(
            piece,
            0.0,
            egui::TextFormat {
                font_id: font.clone(),
                color: color32(theme.color(hs.class)),
                ..Default::default()
            },
        );
    }
    // Fallback: if a highlighter yielded nothing for non-empty text, show it
    // plain rather than blank (defensive — builtins always cover).
    if job.text.is_empty() && !text.is_empty() {
        job.append(
            text,
            0.0,
            egui::TextFormat {
                font_id: font,
                color: color32(theme.color(HlClass::Plain)),
                ..Default::default()
            },
        );
    }
    job
}
