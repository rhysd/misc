use iced::alignment::Horizontal;
use iced::widget::{button, column, container, text, Button, Container};
use iced::window::Settings;
use iced::{application, Length};

const WIDTH: f32 = 200.0;

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i32,
}

fn centered_button(txt: &str) -> Button<Message> {
    button(text(txt).width(Length::Fill).center()).width(Length::Fill)
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Container<Message> {
        let increment = centered_button("+").on_press(Message::Increment);
        let decrement = centered_button("-").on_press(Message::Decrement);
        let count = text(self.value);
        let ui = column![increment, count, decrement]
            .align_x(Horizontal::Center)
            .width(40.0);
        container(ui).center(WIDTH)
    }
}

fn main() -> iced::Result {
    let size = (WIDTH, WIDTH).into();
    application("Counter", Counter::update, Counter::view)
        .window(Settings {
            size,
            max_size: Some(size),
            ..Default::default()
        })
        .run()
}
