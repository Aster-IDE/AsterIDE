use std::sync::RwLock;

pub mod editor;
pub mod general;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    General,
    Editor,
}

/// Owner of data on what page you are on
static CURRENT: RwLock<Page> = RwLock::new(Page::General);

/// Sets page data for which page view to be on
pub fn set_page(id: Page) {
    tracing::debug!("Setting RwLock to {id:?}");
    *CURRENT.write().unwrap() = id;
}

pub fn current_page() -> Page {
    *CURRENT.read().unwrap()
}
