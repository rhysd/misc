use iced::alignment::{Horizontal, Vertical};
use iced::keyboard::{key, Event as KeyEvent, Key};
use iced::widget::image::{Handle, Image, Viewer};
use iced::widget::{button, container, text, Column, Container, Row};
use iced::window::settings::{PlatformSpecific, Settings};
use iced::{application, event, Event, Length::Fill, Padding, Subscription, Theme};
use std::env;
use std::fs;
use std::path::PathBuf;

const BORDER_WIDTH: f32 = 2.0;

#[derive(Debug, Clone, Copy)]
enum Message {
    NextImage,
    PrevImage,
    SetCurrent(usize),
}

struct App {
    current: usize,
    handles: Vec<Handle>,
}

impl Default for App {
    fn default() -> Self {
        let mut handles = vec![];
        for path in env::args_os().skip(1) {
            let path = PathBuf::from(path);
            if path.is_file() {
                handles.push(Handle::from_path(path));
            } else if path.is_dir() {
                handles.extend(
                    fs::read_dir(&path)
                        .unwrap()
                        .flatten()
                        .map(|e| e.path())
                        .filter(|p| p.is_file())
                        .map(Handle::from_path),
                );
            }
        }
        Self {
            current: 0,
            handles,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::NextImage if self.current < self.handles.len() - 1 => self.current += 1,
            Message::PrevImage if self.current > 0 => self.current -= 1,
            Message::SetCurrent(idx) if idx < self.handles.len() => self.current = idx,
            _ => {}
        }
    }

    fn thumbnail(&self) -> Column<Message> {
        const WIDTH: f32 = 150.0;
        const MARGIN: f32 = 10.0;
        const IMAGE_WIDTH: f32 = WIDTH - MARGIN * 2.0 - BORDER_WIDTH * 2.0;

        let start = self.current.saturating_sub(2);
        let end = self.handles.len().min(start + 5);
        let padding = Padding {
            top: MARGIN + 20.0,
            left: MARGIN,
            right: MARGIN,
            bottom: MARGIN,
        };
        let mut col = Column::new()
            .spacing(MARGIN)
            .padding(padding)
            .width(WIDTH)
            .align_x(Horizontal::Center);

        for idx in start..end {
            let image = Image::new(&self.handles[idx]);
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
        if let Some(handle) = self.handles.get(self.current) {
            container(Viewer::new(handle).width(Fill).height(Fill)).center(Fill)
        } else {
            container(text("No image")).center(Fill)
        }
    }

    fn view(&self) -> Row<Message> {
        let mut row = Row::new().align_y(Vertical::Center).padding(8.0);
        if self.handles.len() > 1 {
            row = row.push(self.thumbnail());
        }
        row.push(self.viewer())
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _id| match event {
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
    application("Image Viewer", App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .window_size((800.0, 600.0))
        .window(Settings {
            platform_specific: PlatformSpecific {
                title_hidden: true,
                titlebar_transparent: true,
                fullsize_content_view: true,
            },
            ..Default::default()
        })
        .run()
}
