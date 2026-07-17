use std::sync::RwLock;

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
static CURRENT: RwLock<Page> = RwLock::new(Page::Home);

/// Sets page data for which page view to be on
pub fn set_page(id: Page) {
    tracing::debug!("Setting RwLock to {id:?}");
    *CURRENT.write().unwrap() = id;
}

pub fn current_page() -> Page {
    *CURRENT.read().unwrap()
}
