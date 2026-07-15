use std::sync::{Mutex, OnceLock};

pub mod home;
pub mod search;
pub mod settings;
pub mod workspace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Search,
    Workspace,
    Settings,
}

/// Owner of data on what page you are on
fn current() -> &'static Mutex<Page> {
    static CURRENT: OnceLock<Mutex<Page>> = OnceLock::new();
    CURRENT.get_or_init(|| Mutex::new(Page::Home))
}

/// Sets page data for which page view to be on
pub fn set_page(id: Page) {
    *current().lock().unwrap() = id;
}

pub fn current_page() -> Page {
    *current().lock().unwrap()
}
