mod pages;
mod sidebar;
use iced::{Element, Subscription, Task, widget::row};

use crate::settings::pages::Page;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Settings {
    sidebar: sidebar::Sidebar,
    general: pages::general::General,
    editor: pages::editor::Editor,
}

#[derive(Debug, Clone)]
pub enum Message {
    Sidebar(sidebar::Message),
    General(pages::general::Message),
    Editor(pages::editor::Message),
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            self.general.subscription().map(Message::General),
            self.editor.subscription().map(Message::Editor),
        ])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Sidebar(msg) => self.sidebar.update(msg).map(Message::Sidebar),
            Message::General(msg) => self.general.update(msg).map(Message::General),
            Message::Editor(msg) => self.editor.update(msg).map(Message::Editor),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let page: Element<'_, Message> = match pages::current_page() {
            Page::General => self.general.view().map(Message::General),
            Page::Editor => self.editor.view().map(Message::Editor),
        };

        row![self.sidebar.view().map(Message::Sidebar), page].into()
    }
}
