use iced::alignment::{Horizontal, Vertical};
use iced::keyboard::{key, Event as KeyEvent, Key};
use iced::widget::image::{Handle, Image, Viewer};
use iced::widget::{button, column, container, text, Column, Container, Row};
use iced::{application, event, Event, Length::Fill, Subscription, Theme};
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

    fn thumbnail(&self) -> Row<Message> {
        const HEIGHT: f32 = 100.0;
        const MARGIN: f32 = 10.0;
        const IMAGE_HEIGHT: f32 = HEIGHT - MARGIN * 2.0 - BORDER_WIDTH * 2.0;

        let start = self.current.saturating_sub(2);
        let end = self.handles.len().min(start + 5);
        let mut row = Row::new()
            .spacing(MARGIN)
            .padding(MARGIN)
            .height(HEIGHT)
            .align_y(Vertical::Center);

        for idx in start..end {
            let image = Image::new(&self.handles[idx]);
            let image = button(image).on_press(Message::SetCurrent(idx));
            row = if idx == self.current {
                row.push(image.padding(BORDER_WIDTH))
            } else {
                row.push(image.padding(0.0).height(IMAGE_HEIGHT))
            };
        }

        row
    }

    fn viewer(&self) -> Container<Message> {
        if let Some(handle) = self.handles.get(self.current) {
            container(Viewer::new(handle).width(Fill).height(Fill)).center(Fill)
        } else {
            container(text("No image")).center(Fill)
        }
    }

    fn view(&self) -> Column<Message> {
        let mut col = column![self.viewer()].align_x(Horizontal::Center);
        if self.handles.len() > 1 {
            col = col.push(self.thumbnail());
        }
        col
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _id| match event {
            Event::Keyboard(KeyEvent::KeyPressed {
                key: Key::Named(key::Named::ArrowLeft),
                ..
            }) => Some(Message::PrevImage),
            Event::Keyboard(KeyEvent::KeyPressed {
                key: Key::Named(key::Named::ArrowRight),
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
        .run()
}
