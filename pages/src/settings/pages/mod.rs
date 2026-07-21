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
