use config::theme::ThemeOption;
use iced::{
    Element, Length, Subscription, Task, Theme,
    widget::{Space, column, pick_list, row, text},
};

#[derive(Debug, Clone, PartialEq)]
pub struct General {
    selected_theme: String,
}

impl Default for General {
    fn default() -> Self {
        Self {
            selected_theme: config::get().appearance.theme.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(String),
}

impl General {
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ThemeSelected(key) => {
                println!("selected theme: {key:?}");
                self.selected_theme = key.clone();

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let config = config::get();
        let options = config::spec::theme_options(&config);

        let selected = options
            .iter()
            .find(|opt| opt.key == self.selected_theme)
            .cloned();

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
                pick_list(options, selected, |opt: ThemeOption| {
                    Message::ThemeSelected(opt.key)
                })
            ]
        ]
        .padding(25)
        .into()
    }
}
