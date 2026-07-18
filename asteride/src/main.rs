mod app_icon;
mod sidebar;
use std::path::PathBuf;

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
        let config_reader = config::get();
        config_reader.appearance.resolve_theme()
    }

    fn scale_factor(&self) -> f32 {
        let config_reader = config::get();

        config_reader.appearance.scale.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            self.home.subscription().map(Message::Home),
            self.workspace.subscription().map(Message::Workspace),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Sidebar(msg) => self.sidebar.update(msg).map(Message::Sidebar),
            Message::BottomArea(msg) => self.bottom_area.update(msg).map(Message::BottomArea),
            Message::Home(msg) => self.home.update(msg).map(Message::Home),
            Message::Search(msg) => self.search.update(msg).map(Message::Search),
            Message::Workspace(msg) => self.workspace.update(msg).map(Message::Workspace),
            Message::Settings(msg) => self.settings.update(msg).map(Message::Settings),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let page: Element<'_, Message> = match pages::current_page() {
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
            home: pages::home::Home::new(),
            search: pages::search::Search::new(),
            workspace: pages::workspace::Workspace::new(),
            settings: pages::settings::Settings::new(),
        }
    }
}

struct Args {
    config_path: Option<PathBuf>,
    version: bool,
    silent: bool,
}

fn parse_args() -> Args {
    let mut args = pico_args::Arguments::from_env();

    Args {
        config_path: args.opt_value_from_str(["-c", "--config"]).unwrap(),
        version: args.contains(["-v", "--version"]),
        silent: args.contains(["-s", "--silent"]),
    }
}

pub fn main() -> iced::Result {
    let args = parse_args();

    if args.version {
        print!("AsterIDE v{}", env!("CARGO_PKG_VERSION"));
        return iced::Result::Ok(());
    }

    if !args.silent {
        tracing::subscriber::set_global_default(logging_subscriber::SimpleSubscriber).unwrap();
    }

    config::init_ring(args.config_path);

    let window_settings = window::Settings {
        // TODO: Create own titlebar later
        decorations: true,
        transparent: true,
        icon: app_icon::APP_ICON.clone(),
        ..Default::default()
    };

    let boot_settings = iced::Settings {
        fonts: vec![LUCIDE_FONT_BYTES.into()],
        ..Default::default()
    };

    tracing::info!("Init iced app");

    iced::application(AsterIDE::new, AsterIDE::update, AsterIDE::view)
        .subscription(AsterIDE::subscription)
        .title("AsterIDE")
        .window(window_settings)
        .scale_factor(AsterIDE::scale_factor)
        .theme(AsterIDE::theme)
        .settings(boot_settings)
        .run()
}
