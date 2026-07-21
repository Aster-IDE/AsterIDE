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

use iced::{Element, Subscription, Task, widget::text};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Search {}

#[derive(Debug, Clone)]
pub enum Message {}

impl Search {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // _ => Task::none(),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        text("Search page").into()
    }
}
