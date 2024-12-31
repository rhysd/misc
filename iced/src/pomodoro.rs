use iced::alignment::Horizontal;
use iced::time::{self, Duration};
use iced::widget::{button, column, container, row, text, Button, Container, Row};
use iced::window::{Level, Settings};
use iced::{application, Color, Length, Subscription, Theme};

const WIDTH: f32 = 200.0;
const HEIGHT: f32 = 120.0;

#[derive(Debug, Clone, Copy)]
enum Message {
    Start,
    Tick,
    Pause,
    Resume,
    Reset,
}

#[derive(Default, Clone, Copy)]
enum Pomodoro {
    #[default]
    Ready,
    Running(u16),
    Pending(u16),
    Done,
}

impl Pomodoro {
    fn update(&mut self, message: Message) {
        *self = match (message, *self) {
            (Message::Start, _) => Self::Running(1800),
            (Message::Tick, Self::Running(c)) if c > 1 => Self::Running(c - 1),
            (Message::Tick, Self::Running(_)) => Self::Done,
            (Message::Pause, Self::Running(c)) => Self::Pending(c),
            (Message::Resume, Self::Pending(c)) => Self::Running(c),
            (Message::Reset, _) => Self::Ready,
            (_, cur) => cur,
        };
    }

    fn timer_color(&self) -> Option<Color> {
        match self {
            Self::Ready | Self::Pending(_) => Some([0.7, 0.7, 0.7, 1.0].into()),
            Self::Running(_) => None,
            Self::Done => Some([1.0, 0.0, 0.0, 1.0].into()),
        }
    }

    fn count(&self) -> u16 {
        match *self {
            Self::Ready => 1800,
            Self::Running(c) | Self::Pending(c) => c,
            Self::Done => 0,
        }
    }

    fn timer(&self) -> Row<Message> {
        let count = self.count();
        let min = text!("{:02}", count / 60);
        let sec = text!("{:02}", count % 60);
        let color = self.timer_color();
        row![
            min.color_maybe(color).size(40.0).width(50.0),
            text(":").color_maybe(color).size(40.0),
            sec.color_maybe(color).size(40.0).width(50.0),
        ]
    }

    fn buttons(&self) -> Row<Message> {
        fn btn(txt: &str, msg: Message) -> Button<Message> {
            button(text(txt).width(Length::Fill).center())
                .width(Length::Fill)
                .padding(4.0)
                .on_press(msg)
        }

        let left = match self {
            Self::Ready => btn("Start", Message::Start),
            Self::Running(_) => btn("Pause", Message::Pause),
            Self::Pending(_) => btn("Resume", Message::Resume),
            Self::Done => btn("Start", Message::Start),
        };
        let right = btn("Reset", Message::Reset);
        row![left, right].spacing(8.0)
    }

    fn view(&self) -> Container<Message> {
        let timer = self.timer();
        let buttons = self.buttons();
        let ui = column![timer, buttons]
            .align_x(Horizontal::Center)
            .padding(8.0)
            .width(Length::Fill);
        container(ui).center(WIDTH)
    }

    fn subscription(&self) -> Subscription<Message> {
        match self {
            Self::Running(_) => time::every(Duration::from_secs(1)).map(|_| Message::Tick),
            _ => Subscription::none(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }
}

fn main() -> iced::Result {
    let size = (WIDTH, HEIGHT).into();
    application("Pomodoro", Pomodoro::update, Pomodoro::view)
        .subscription(Pomodoro::subscription)
        .theme(Pomodoro::theme)
        .window(Settings {
            size,
            max_size: Some(size),
            level: Level::AlwaysOnTop,
            ..Default::default()
        })
        .run()
}
