mod app_icon;
mod sidebar;
use elements::bottom_area;
use iced::{
    Element, Subscription, Task,
    theme::Theme,
    widget::{column, row},
    window,
};
use lucide_icons::LUCIDE_FONT_BYTES;

struct AsterIDE {
    sidebar: sidebar::Sidebar,
    page: pages::Page,
    home: pages::home::Home,
    search: pages::search::Search,
    workspace: pages::workspace::Workspace,
    settings: pages::settings::Settings,
    bottom_area: elements::bottom_area::BottomArea,
}

#[derive(Debug, Clone)]
enum Message {
    Home(pages::home::Message),
    Workspace(pages::workspace::Message),
    Settings(pages::settings::Message),
    Search(pages::search::Message),
    Sidebar(sidebar::Message),
    BottomArea(bottom_area::Message),
}

impl AsterIDE {
    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            self.home.subscription().map(Message::Home),
            self.workspace.subscription().map(Message::Workspace),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Sidebar(msg) => {
                let (task, event) = self.sidebar.update(msg);
                if let sidebar::Event::OpenPage(id) = event {
                    self.page = id;
                }
                task.map(Message::Sidebar)
            }
            Message::BottomArea(msg) => self.bottom_area.update(msg).map(Message::BottomArea),
            Message::Home(msg) => self.home.update(msg).map(Message::Home),
            Message::Search(msg) => self.search.update(msg).map(Message::Search),
            Message::Workspace(msg) => self.workspace.update(msg).map(Message::Workspace),
            Message::Settings(msg) => self.settings.update(msg).map(Message::Settings),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let page: Element<'_, Message> = match self.page {
            pages::Page::Home => self.home.view().map(Message::Home),
            pages::Page::Search => self.search.view().map(Message::Search),
            pages::Page::Workspace => self.workspace.view().map(Message::Workspace),
            pages::Page::Settings => self.settings.view().map(Message::Settings),
        };

        column![
            row![self.sidebar.view().map(Message::Sidebar), page],
            self.bottom_area.view().map(Message::BottomArea)
        ]
        .into()
    }

    fn new() -> Self {
        Self {
            sidebar: sidebar::Sidebar::new(),
            bottom_area: bottom_area::BottomArea::new(),
            page: pages::Page::Home,
            home: pages::home::Home::new(),
            search: pages::search::Search::new(),
            workspace: pages::workspace::Workspace::new(),
            settings: pages::settings::Settings::new(),
        }
    }
}

pub fn main() -> iced::Result {
    let window_settings = window::Settings {
        // TODO: Looking to create own titlebar stuff later
        decorations: true,
        // INFO: will ready for cool looks
        transparent: true,
        icon: app_icon::APP_ICON.clone(),
        ..Default::default()
    };

    let boot_settings = iced::Settings {
        fonts: vec![LUCIDE_FONT_BYTES.into()],
        ..Default::default()
    };

    iced::application(AsterIDE::new, AsterIDE::update, AsterIDE::view)
        .subscription(AsterIDE::subscription)
        .title("AsterIDE")
        .window(window_settings)
        .theme(AsterIDE::theme)
        .settings(boot_settings)
        .run()
}
