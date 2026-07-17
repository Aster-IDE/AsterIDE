use iced::{
    Element,
    Length,
    Subscription,
    Task,
    Theme,
    widget::{
        Space,
        column,
        pick_list,
        row,
        text,
    },
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct General {
    selected_theme: Option<iced::Theme>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(iced::Theme),
}

impl General {
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ThemeSelected(theme) => {
                println!("selected theme: {theme:?}");
                self.selected_theme = Some(theme);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            text("General").size(25),
            Space::new().height(Length::Fixed(10.0)),
            row![
                column![
                    text("Theme"),
                    text("Colorscheme provided to whole app")
                        .size(13)
                        .style(|theme: &Theme| {
                            let palette = theme.extended_palette();

                            text::Style {
                                color: Some(palette.secondary.weak.color),
                            }
                        })
                ],
                Space::new().width(Length::Fill),
                pick_list(
                    iced::Theme::ALL,
                    self.selected_theme.clone(),
                    Message::ThemeSelected
                )
            ]
        ]
        .padding(25)
        .into()
    }
}
