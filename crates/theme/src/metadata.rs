use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub author: &'static str,
    pub version: &'static str,
    pub is_dark: bool,
    pub family: &'static str,
}
