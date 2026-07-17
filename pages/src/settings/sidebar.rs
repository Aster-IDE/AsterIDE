use crate::settings::pages::{Page, set_page};
use elements::bottom_area;
use iced::widget::{button, column, container, mouse_area, row, text};
use iced::{Element, Font, Length, Task, Theme};
use lucide_icons::Icon;

const LUCIDE_FONT: Font = Font::with_name("lucide");

#[derive(Default, Debug, Clone, PartialEq, Eq)]
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
            Message::ContextButtonEnter(t, d) => {
                bottom_area::announce_ctx(t, d);
            }
            Message::ContextButtonExit => {
                bottom_area::clear_ctx();
            }

            Message::SwitchPage(p) => set_page(p),
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                nav_button(
                    icon_text(Icon::AlignJustify),
                    "General",
                    "Settings for most of generic things",
                    Page::General
                ),
                nav_button(
                    icon_text(Icon::Pencil),
                    "Editor",
                    "Settings for text editor field",
                    Page::Editor
                ),
            ]
            .spacing(10)
            .padding(10)
            .width(150)
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
}

fn nav_button<'a>(
    icon: impl Into<Element<'static, Message>>,
    title: &'a str,
    description: &'a str,
    page: Page,
) -> Element<'a, Message> {
    mouse_area(
        button(row![icon.into(), text(title).height(Length::Fill).center()].spacing(10))
            .on_press(Message::SwitchPage(page))
            .width(Length::Fill)
            .height(28)
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
                        radius: 2.0.into(),
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
        .size(16.0)
        .height(Length::Fill)
        .center()
        .into()
}
