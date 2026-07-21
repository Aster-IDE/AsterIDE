/*
 * This file is part of AsterIDE.
 *
 * Copyright (c) 2026 AsterIDE
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
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilesSettings {
    pub recent_files_limit: usize,
    pub recent_projects_limit: usize,
    pub auto_detect_indentation: bool,
    pub trim_trailing_whitespace: bool,
    pub insert_final_newline: bool,
    pub trim_auto_whitespace: bool,
    pub pinned_files: Vec<PathBuf>,
    pub file_associations: HashMap<String, String>,
    pub default_file_encoding: String,
    pub auto_reload_files: bool,
    pub confirm_before_save: bool,

    #[serde(skip)]
    pub new_file_ext_input: String,
    #[serde(skip)]
    pub new_file_lang_input: String,
}

impl Default for FilesSettings {
    fn default() -> Self {
        Self {
            recent_files_limit: 5,
            recent_projects_limit: 5,
            auto_detect_indentation: true,
            trim_trailing_whitespace: false,
            insert_final_newline: false,
            trim_auto_whitespace: false,
            pinned_files: Vec::new(),
            file_associations: HashMap::new(),
            default_file_encoding: "utf8".to_string(),
            auto_reload_files: false,
            confirm_before_save: false,
            new_file_ext_input: String::new(),
            new_file_lang_input: String::new(),
        }
    }
}
