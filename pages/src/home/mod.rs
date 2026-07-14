use iced::{
    Element, Font, Length, Subscription, Task, Theme,
    widget::{button, column, container, row, text},
};
use lucide_icons::Icon;
use rfd::FileDialog;

const LUCIDE_FONT: Font = Font::with_name("lucide");

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Home {}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFolder,
    OpenFile,
    NewFile,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenFolder => {
                if let Some(dir) = FileDialog::new().pick_folder() {
                    println!("{dir:#?}")
                };

                Task::none()
            }
            Message::OpenFile => {
                if let Some(file) = FileDialog::new().pick_file() {
                    println!("{file:#?}")
                };

                Task::none()
            }
            Message::NewFile => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                column![
                    text("AsterIDE")
                        .size(44)
                        .width(Length::Fill)
                        .center()
                        .style(|theme: &Theme| {
                            let palette = theme.extended_palette();

                            text::Style {
                                color: Some(palette.primary.base.color),
                            }
                        }),
                    text("A simple text editor written in Rust")
                        .size(18)
                        .width(Length::Fill)
                        .center()
                ],
                container(
                    column![
                        nav_button(Icon::FolderOpen, "Open Folder", Message::OpenFolder),
                        nav_button(Icon::FolderOpen, "Open File", Message::OpenFile),
                        nav_button(Icon::FileText, "New File", Message::NewFile),
                    ]
                    .width(250)
                    .spacing(10)
                )
                .center_x(Length::Fill),
            ]
            .spacing(25),
        )
        .center(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

fn nav_button<'a>(icon: Icon, display: &'a str, message: Message) -> Element<'a, Message> {
    button(row![icon_text(icon), text(display).height(Length::Fill).center()].spacing(10))
        .on_press(message)
        .padding(10)
        .height(45)
        .width(Length::Fill)
        .style(|theme, status| {
            let mut style = button::primary(theme, status);
            style.border = iced::Border {
                radius: 5.0.into(),
                ..style.border
            };
            style
        })
        .into()
}

fn icon_text<'a, Message: 'a>(icon: Icon) -> Element<'a, Message> {
    text(char::from(icon).to_string())
        .font(LUCIDE_FONT)
        .size(22.0)
        .width(25)
        .height(25)
        .center()
        .into()
}
