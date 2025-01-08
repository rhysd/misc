use iced::alignment::{Horizontal, Vertical};
use iced::keyboard::{key, Event as KeyEvent, Key};
use iced::widget::image::{Handle, Image, Viewer};
use iced::widget::{button, container, text, Column, Container, Row};
use iced::window::{self, Event as WindowEvent, Id as WindowId};
use iced::{application, event, Event, Length::Fill, Subscription, Task, Theme};
use std::env;
use std::fs;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

const BORDER_WIDTH: f32 = 2.0;

#[derive(Debug, Clone, Copy)]
enum Message {
    NextImage,
    PrevImage,
    SetCurrent(usize),
    Init(WindowId),
}

struct File {
    handle: Handle,
    path: String,
}

impl File {
    fn new(path: PathBuf, home: Option<&Path>) -> Self {
        let canon = path.canonicalize().unwrap_or_else(|_| path.clone());
        let handle = Handle::from_path(path);
        let path = home
            .and_then(|p| canon.strip_prefix(p).ok())
            .map(|p| format!("~{}{}", MAIN_SEPARATOR, p.display()))
            .unwrap_or_else(|| canon.into_os_string().to_string_lossy().into_owned());
        File { handle, path }
    }
}

struct App {
    current: usize,
    files: Vec<File>,
    init: bool,
}

impl Default for App {
    fn default() -> Self {
        let home = dirs::home_dir();
        let home = home.as_deref();

        let mut files = vec![];
        for path in env::args_os().skip(1) {
            let path = PathBuf::from(path);
            if path.is_file() {
                files.push(File::new(path, home));
            } else if path.is_dir() {
                files.extend(
                    fs::read_dir(&path)
                        .unwrap()
                        .flatten()
                        .map(|e| e.path())
                        .filter(|p| p.is_file())
                        .map(|p| File::new(p, home)),
                );
            }
        }

        Self {
            current: 0,
            files,
            init: false,
        }
    }
}

impl App {
    fn title(&self) -> String {
        if let Some(file) = self.files.get(self.current) {
            file.path.clone()
        } else {
            "Image Viewer".to_string()
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NextImage if self.current < self.files.len() - 1 => self.current += 1,
            Message::PrevImage if self.current > 0 => self.current -= 1,
            Message::SetCurrent(idx) if idx < self.files.len() => self.current = idx,
            Message::Init(id) if !self.init => {
                self.init = true;
                return window::maximize(id, true);
            }
            _ => {}
        }
        Task::none()
    }

    fn thumbnail(&self) -> Column<Message> {
        const WIDTH: f32 = 150.0;
        const MARGIN: f32 = 10.0;
        const IMAGE_WIDTH: f32 = WIDTH - MARGIN * 2.0 - BORDER_WIDTH * 2.0;

        let start = self.current.saturating_sub(2);
        let end = self.files.len().min(start + 5);
        let mut col = Column::new()
            .spacing(MARGIN)
            .padding(MARGIN)
            .width(WIDTH)
            .align_x(Horizontal::Center);

        for idx in start..end {
            let image = Image::new(&self.files[idx].handle);
            let image = button(image).on_press(Message::SetCurrent(idx));
            col = if idx == self.current {
                col.push(image.padding(BORDER_WIDTH))
            } else {
                col.push(image.padding(0.0).width(IMAGE_WIDTH))
            };
        }

        col
    }

    fn viewer(&self) -> Container<Message> {
        if let Some(file) = self.files.get(self.current) {
            container(Viewer::new(&file.handle).width(Fill).height(Fill)).center(Fill)
        } else {
            container(text("No image")).center(Fill)
        }
    }

    fn view(&self) -> Row<Message> {
        let mut row = Row::new().align_y(Vertical::Center);
        if self.files.len() > 1 {
            row = row.push(self.thumbnail());
        }
        row.push(self.viewer())
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, id| match event {
            Event::Window(WindowEvent::Opened { .. }) => Some(Message::Init(id)),
            Event::Keyboard(KeyEvent::KeyPressed {
                key: Key::Named(key::Named::ArrowLeft | key::Named::ArrowUp),
                ..
            }) => Some(Message::PrevImage),
            Event::Keyboard(KeyEvent::KeyPressed {
                key: Key::Named(key::Named::ArrowRight | key::Named::ArrowDown),
                ..
            }) => Some(Message::NextImage),
            _ => None,
        })
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn main() -> iced::Result {
    application(App::title, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .window_size((800.0, 600.0))
        .run()
}
