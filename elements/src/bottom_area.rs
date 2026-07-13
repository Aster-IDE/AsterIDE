use std::sync::{Mutex, OnceLock};

use iced::widget::{Space, container, row, text};
use iced::{Element, Length, Padding, Task, Theme};

#[derive(Default)]
struct ContextState {
    title: String,
    description: String,
}

fn state() -> &'static Mutex<ContextState> {
    static STATE: OnceLock<Mutex<ContextState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(ContextState::default()))
}

pub fn announce_ctx(title: impl Into<String>, description: impl Into<String>) {
    let mut s = state().lock().unwrap();
    s.title = title.into();
    s.description = description.into();
}

pub fn clear_ctx() {
    let mut s = state().lock().unwrap();
    s.title.clear();
    s.description.clear();
}

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextType {
    Attention,
    Readable,
    Muted,
}

#[derive(Default)]
pub struct BottomArea {}

impl BottomArea {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {}
        // Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let (title, description) = {
            let s = state().lock().unwrap();
            (s.title.clone(), s.description.clone())
        };

        container(
            row![
                bar_text(
                    format!("AsterIDE v{}", env!("CARGO_PKG_VERSION")).into(),
                    TextType::Attention
                ),
                bar_text(title, TextType::Readable),
                bar_text(description, TextType::Muted),
                Space::new().width(Length::Fill)
            ]
            .spacing(10),
        )
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();

            container::Style {
                background: Some(palette.background.weakest.color.into()),
                ..Default::default()
            }
        })
        .padding(Padding {
            left: 15.0,
            right: 15.0,
            ..Default::default()
        })
        .center_y(Length::Fill)
        .width(Length::Fill)
        .height(45)
        .into()
    }

    pub fn new() -> Self {
        Self {}
    }
}

pub fn bar_text(display: String, text_type: TextType) -> Element<'static, Message> {
    text(display)
        .style(move |theme: &Theme| {
            let palette = theme.extended_palette();

            text::Style {
                color: Some(match text_type {
                    TextType::Attention => palette.primary.strong.color.into(),
                    TextType::Readable => palette.secondary.strong.color.into(),
                    TextType::Muted => palette.secondary.weak.color.into(),
                }),
            }
        })
        .size(17)
        .into()
}
