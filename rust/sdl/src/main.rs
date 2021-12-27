use sdl2::event::Event;
use sdl2::joystick::HatState;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::time::Instant;

const CELL_PX: u32 = 10;
const BLACK: Color = Color::RGB(0, 0, 0);
const GRAY: Color = Color::RGB(192, 192, 192);
const WHITE: Color = Color::RGB(255, 255, 255);
const RED: Color = Color::RGB(128, 0, 0);
const TICK_FRAMES: u8 = 10;
const FOCUS_FRAMES: u8 = 60;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Off,
    On,
}

impl Cell {
    fn color(self) -> Color {
        match self {
            Cell::On => BLACK,
            Cell::Off => WHITE,
        }
    }

    fn flip(&mut self) {
        *self = match *self {
            Cell::On => Cell::Off,
            Cell::Off => Cell::On,
        };
    }

    fn random() -> Self {
        if rand::random() {
            Self::On
        } else {
            Self::Off
        }
    }
}

#[derive(Clone, Copy)]
enum Horizontal {
    Left(u8),
    Right(u8),
    Neutral,
}
impl Default for Horizontal {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Clone, Copy)]
enum Vertical {
    Up(u8),
    Down(u8),
    Neutral,
}
impl Default for Vertical {
    fn default() -> Self {
        Self::Neutral
    }
}

struct Game {
    cells: Box<[Box<[Cell]>]>,
    gen_frames: Option<u8>,
    focus_frames: Option<u8>,
    focus: (usize, usize),
    stick: (Horizontal, Vertical),
}

impl Game {
    fn new((w, h): (usize, usize)) -> Self {
        let w = (w / 2) / (CELL_PX as usize + 1);
        let h = (h / 2) / (CELL_PX as usize + 1);
        let cells = vec![vec![Cell::Off; w].into_boxed_slice(); h].into_boxed_slice();

        Self {
            cells,
            gen_frames: None,
            focus_frames: None,
            focus: (0, 0),
            stick: Default::default(),
        }
    }

    fn window_size(&self) -> (u32, u32) {
        let h = self.cells.len() as u32 * (1 + CELL_PX) + 1;
        let w = self.cells[0].len() as u32 * (1 + CELL_PX) + 1;
        (w, h)
    }

    fn tick(&mut self) -> bool {
        if let Some(mut f) = self.gen_frames {
            f -= 1;
            if f == 0 {
                self.gen_frames = Some(TICK_FRAMES);
                true
            } else {
                self.gen_frames = Some(f);
                false
            }
        } else {
            false
        }
    }

    fn toggle(&mut self) {
        self.gen_frames = self.gen_frames.is_none().then(|| TICK_FRAMES);
    }

    fn focus(&mut self, x_px: usize, y_px: usize) {
        let stride = CELL_PX as usize + 1;
        let (x, y) = (x_px / stride, y_px / stride);
        self.focus = (x, y);
        self.focus_frames = Some(FOCUS_FRAMES);
    }

    fn move_focus(&mut self, hori: Horizontal, vert: Vertical) {
        use Horizontal::*;
        use Vertical::*;
        let (w, h) = (self.cells[0].len(), self.cells.len());
        let (x, y) = self.focus;
        let x = match hori {
            Left(dx) if x >= dx as usize => x - dx as usize,
            Right(dx) if w > x + dx as usize => x + dx as usize,
            _ => x,
        };
        let y = match vert {
            Up(dy) if y >= dy as usize => y - dy as usize,
            Down(dy) if h > y + dy as usize => y + dy as usize,
            _ => y,
        };
        self.focus = (x, y);
        self.focus_frames = Some(FOCUS_FRAMES);
    }

    fn stick_horizontal(&mut self, h: Horizontal) {
        self.stick.0 = h;
    }
    fn stick_vertical(&mut self, v: Vertical) {
        self.stick.1 = v;
    }

    fn flip_cell_at(&mut self, x_px: usize, y_px: usize) {
        let stride = CELL_PX as usize + 1;
        let (x, y) = (x_px / stride, y_px / stride);
        self.cells[y][x].flip();
    }

    fn flip_cell(&mut self) {
        let (x, y) = self.focus;
        self.cells[y][x].flip();
        self.focus_frames = Some(FOCUS_FRAMES);
    }

    fn reset(&mut self) {
        for xs in self.cells.iter_mut() {
            xs.fill(Cell::Off);
        }
    }

    fn random(&mut self) {
        for xs in self.cells.iter_mut() {
            for c in xs.iter_mut() {
                *c = Cell::random();
            }
        }
    }

    fn live_cells(&self, x: usize, y: usize) -> u8 {
        let xs = [
            (x > 0).then(|| x - 1),
            Some(x),
            (self.cells[0].len() > x + 1).then(|| x + 1),
        ];
        let ys = [
            (y > 0).then(|| y - 1),
            Some(y),
            (self.cells.len() > y + 1).then(|| y + 1),
        ];

        let mut count = 0;
        for y in ys.into_iter().flatten() {
            for x in xs.iter().copied().flatten() {
                if matches!(self.cells[y][x], Cell::On) {
                    count += 1;
                }
            }
        }
        if self.cells[y][x] == Cell::On {
            count -= 1;
        }
        count
    }

    fn update(&mut self) {
        self.focus_frames = self.focus_frames.and_then(|f| f.checked_sub(1));
        match self.stick {
            (Horizontal::Neutral, Vertical::Neutral) => {}
            (h, v) => self.move_focus(h, v),
        }

        if !self.tick() {
            return;
        }
        let mut flips = vec![];
        for (y, xs) in self.cells.iter().enumerate() {
            for (x, cell) in xs.iter().enumerate() {
                let c = self.live_cells(x, y);
                use Cell::*;
                let next = match *cell {
                    Off if c == 3 => On,
                    Off => Off,
                    On if c == 2 || c == 3 => On,
                    On => Off,
                };
                if next != *cell {
                    flips.push((x, y));
                }
            }
        }
        for (x, y) in flips.into_iter() {
            self.cells[y][x].flip();
        }
    }

    fn draw_scene(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let stride = CELL_PX as i32 + 1;
        let height = self.cells.len() as i32;
        let width = self.cells[0].len() as i32;

        // Draw grids
        canvas.set_draw_color(GRAY);
        for y in 0..=height {
            let y = y * stride;
            canvas.draw_line((0, y), (width * stride, y))?;
        }
        for x in 0..=width {
            let x = x * stride;
            canvas.draw_line((x, 0), (x, height * stride))?;
        }

        // Draw cells
        for (y, xs) in self.cells.iter().enumerate() {
            for (x, c) in xs.iter().enumerate() {
                canvas.set_draw_color(c.color());
                let (x, y) = (x as i32 * stride + 1, y as i32 * stride + 1);
                canvas.fill_rect(Some((x, y, CELL_PX, CELL_PX).into()))?;
            }
        }

        // Draw focus
        if self.focus_frames.is_some() {
            canvas.set_draw_color(RED);
            let (x, y) = self.focus;
            let (x, y) = (x as i32 * stride, y as i32 * stride);
            canvas.draw_rect((x, y, CELL_PX + 2, CELL_PX + 2).into())?;
        }

        Ok(())
    }
}

fn main() -> Result<(), String> {
    let ctx = sdl2::init()?;
    let video = ctx.video()?;
    let mode = video.current_display_mode(0)?;
    let screen = (mode.w as usize, mode.h as usize);
    let mut game = Game::new(screen);
    let (width, height) = game.window_size();

    let joystick = ctx.joystick()?;
    let mut controller = (joystick.num_joysticks()? > 0)
        .then(|| joystick.open(0))
        .transpose()
        .map_err(|e| format!("{}", e))?;

    let window = video
        .window("Life Game for SDL test", width, height)
        .position_centered()
        .build()
        .map_err(|e| format!("{}", e))?;
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| format!("{}", e))?;

    let mut events = ctx.event_pump()?;
    let mut stamp = Instant::now();

    'main: loop {
        for event in events.poll_iter() {
            use Event::*;
            println!("{:?}", event);
            match event {
                Quit { .. } => break 'main,
                MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => game.flip_cell_at(x as usize, y as usize),
                MouseButtonDown {
                    mouse_btn: MouseButton::Right,
                    ..
                } => game.toggle(),
                MouseMotion { x, y, .. } => game.focus(x as usize, y as usize),
                KeyDown {
                    keycode: Some(key), ..
                } => {
                    use Keycode::*;
                    match key {
                        Down | J => game.move_focus(Horizontal::Neutral, Vertical::Down(1)),
                        Up | K => game.move_focus(Horizontal::Neutral, Vertical::Up(1)),
                        Left | H => game.move_focus(Horizontal::Left(1), Vertical::Neutral),
                        Right | L => game.move_focus(Horizontal::Right(1), Vertical::Neutral),
                        Space => game.flip_cell(),
                        Return | Return2 => game.toggle(),
                        Escape | N => game.reset(),
                        M => game.random(),
                        _ => {}
                    }
                }
                JoyHatMotion { state, .. } => {
                    use HatState::*;
                    let (h, v) = match state {
                        Up => (Horizontal::Neutral, Vertical::Up(1)),
                        RightUp => (Horizontal::Right(1), Vertical::Up(1)),
                        Right => (Horizontal::Right(1), Vertical::Neutral),
                        RightDown => (Horizontal::Right(1), Vertical::Down(1)),
                        Down => (Horizontal::Neutral, Vertical::Down(1)),
                        LeftDown => (Horizontal::Left(1), Vertical::Down(1)),
                        Left => (Horizontal::Left(1), Vertical::Neutral),
                        LeftUp => (Horizontal::Left(1), Vertical::Up(1)),
                        Centered => (Horizontal::Neutral, Vertical::Neutral),
                    };
                    game.move_focus(h, v);
                }
                JoyButtonDown { button_idx: 0, .. } => game.flip_cell(), // Cross button of DualShock4
                JoyButtonDown { button_idx: 1, .. } => game.toggle(), // Circle button of DualShock4
                JoyButtonDown { button_idx: 2, .. } => game.random(), // Square button of DualShock4
                JoyButtonDown { button_idx: 3, .. } => game.reset(), // Triangle button of DualShock4
                JoyAxisMotion {
                    axis_idx: 0, value, ..
                } => {
                    let h = match value {
                        -32768..=-30000 => Horizontal::Left(2),
                        -29999..=-10000 => Horizontal::Left(1),
                        -9999..=9999 => Horizontal::Neutral,
                        10000..=29999 => Horizontal::Right(1),
                        30000..=32767 => Horizontal::Right(2),
                    };
                    game.stick_horizontal(h);
                }
                JoyAxisMotion {
                    axis_idx: 1, value, ..
                } => {
                    let v = match value {
                        -32768..=-30000 => Vertical::Up(2),
                        -29999..=-10000 => Vertical::Up(1),
                        -9999..=9999 => Vertical::Neutral,
                        10000..=29999 => Vertical::Down(1),
                        30000..=32767 => Vertical::Down(2),
                    };
                    game.stick_vertical(v);
                }
                JoyDeviceAdded { which: 0, .. } if controller.is_none() => {
                    controller = Some(joystick.open(0).map_err(|e| format!("{}", e))?);
                }
                _ => {}
            }
        }
        game.update();
        game.draw_scene(&mut canvas)?;
        canvas.present();
        let now = Instant::now();
        let _fps = 1000.0 / now.duration_since(stamp).subsec_millis() as f64;
        // println!("{:?} fps", fps);
        stamp = now;
    }

    Ok(())
}
