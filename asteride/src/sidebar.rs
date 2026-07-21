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

use elements::bottom_area;
use iced::widget::{Space, button, column, container, mouse_area, text};
use iced::{Element, Font, Length, Task, Theme};
use lucide_icons::Icon;
use pages::Page;

const LUCIDE_FONT: Font = Font::with_name("lucide");

#[derive(Default)]
pub struct Sidebar {}

#[derive(Debug, Clone)]
pub enum Message {
    ContextButtonEnter(String, String),
    ContextButtonExit,
    SwitchPage(Page),
}

impl Sidebar {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ContextButtonEnter(t, d) => bottom_area::announce_ctx(t, d),
            Message::ContextButtonExit => bottom_area::clear_ctx(),
            Message::SwitchPage(p) => pages::set_page(p),
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                nav_button(
                    icon_text(Icon::Home),
                    "Home",
                    "Opens the startup page",
                    Page::Home
                ),
                nav_button(
                    icon_text(Icon::Hammer),
                    "Workspace",
                    "Opens workspace",
                    Page::Workspace
                ),
                nav_button(
                    icon_text(Icon::Search),
                    "Search",
                    "Searches through the current workspace",
                    Page::Search
                ),
                Space::new().height(Length::Fill),
                nav_button(
                    icon_text(Icon::Settings),
                    "Settings",
                    "Opens settings",
                    Page::Settings
                ),
            ]
            .spacing(10)
            .padding(10)
            .width(60)
            .height(Length::Fill),
        )
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();

            container::Style {
                background: Some(palette.background.weakest.color.into()),
                ..Default::default()
            }
        })
        .into()
    }

    pub fn new() -> Self {
        Self {}
    }
}

fn nav_button<'a>(
    icon: impl Into<Element<'static, Message>>,
    title: &'a str,
    description: &'a str,
    page: Page,
) -> Element<'a, Message> {
    mouse_area(
        button(icon.into())
            .on_press(Message::SwitchPage(page))
            .width(Length::Fill)
            .height(40)
            .style(|theme: &Theme, state: button::Status| {
                let palette = theme.extended_palette();
                let hovered = state == button::Status::Hovered;

                button::Style {
                    background: if hovered {
                        Some(palette.background.weak.color.into())
                    } else {
                        None
                    },
                    text_color: palette.secondary.strong.color,
                    border: iced::Border {
                        radius: 5.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }),
    )
    .on_enter(Message::ContextButtonEnter(
        title.into(),
        description.into(),
    ))
    .on_exit(Message::ContextButtonExit)
    .into()
}

fn icon_text<'a, Message: 'a>(icon: Icon) -> Element<'a, Message> {
    text(char::from(icon).to_string())
        .font(LUCIDE_FONT)
        .size(22.0)
        .width(Length::Fill)
        .height(Length::Fill)
        .center()
        .into()
}
