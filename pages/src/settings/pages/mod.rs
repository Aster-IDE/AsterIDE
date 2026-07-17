use std::sync::{
    Mutex,
    OnceLock,
};

pub mod editor;
pub mod general;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    General,
    Editor,
}

/// Owner of data on what page you are on
fn current() -> &'static Mutex<Page> {
    static CURRENT: OnceLock<Mutex<Page>> = OnceLock::new();
    CURRENT.get_or_init(|| Mutex::new(Page::General))
}

/// Sets page data for which page view to be on
pub fn set_page(id: Page) {
    *current().lock().unwrap() = id;
}

pub fn current_page() -> Page {
    *current().lock().unwrap()
}
