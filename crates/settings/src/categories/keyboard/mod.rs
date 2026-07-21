/*
 * This file is part of AsterIDE.
 *
 * Copyright (c) 2026 playfairs
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub enum KeymapScheme {
    Default,
    VSCode,
    SublimeText,
    Atom,
    Emacs,
}

impl Default for KeymapScheme {
    fn default() -> Self {
        KeymapScheme::VSCode
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyboardSettings {
    pub keymap_scheme: KeymapScheme,
    pub vim_leader_key: String,
    pub multi_cursor_enabled: bool,
    pub bracket_pair_colorization: bool,
    pub suggest_snippets: bool,
    pub quick_suggestions: bool,
}

impl Default for KeyboardSettings {
    fn default() -> Self {
        Self {
            keymap_scheme: KeymapScheme::default(),
            vim_leader_key: ",".to_string(),
            multi_cursor_enabled: true,
            bracket_pair_colorization: true,
            suggest_snippets: true,
            quick_suggestions: true,
        }
    }
}
