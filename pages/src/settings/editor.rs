use iced::{
    Element,
    Subscription,
    Task,
    widget::text,
};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Editor {}

#[derive(Debug, Clone)]
pub enum Message {}

impl Editor {
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // _ => Task::none(),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        text("Settings page (editor)").into()
    }
}
