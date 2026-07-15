mod editor;
mod general;
mod sidebar;
use iced::{Element, Subscription, Task, widget::row};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum SettingsPage {
    #[default]
    General,
    Editor,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    sidebar: sidebar::Sidebar,
    general: general::General,
    editor: editor::Editor,
    page: SettingsPage,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangePage(SettingsPage),
    Sidebar(sidebar::Message),
    General(general::Message),
    Editor(editor::Message),
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
            Message::ChangePage(p) => {
                self.page = p;
                Task::none()
            }
            Message::Sidebar(msg) => {
                let (task, event) = self.sidebar.update(msg);
                if let sidebar::Event::OpenPage(id) = event {
                    self.page = id;
                }
                task.map(Message::Sidebar)
            }
            Message::General(msg) => self.general.update(msg).map(Message::General),
            Message::Editor(msg) => self.editor.update(msg).map(Message::Editor),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let page: Element<'_, Message> = match self.page {
            SettingsPage::General => self.general.view().map(Message::General),
            SettingsPage::Editor => self.editor.view().map(Message::Editor),
        };

        row![self.sidebar.view().map(Message::Sidebar), page].into()
    }
}
