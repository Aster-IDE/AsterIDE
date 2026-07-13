pub mod home;
pub mod search;
pub mod settings;
pub mod workspace;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Home,
    Search,
    Workspace,
    Settings,
}
