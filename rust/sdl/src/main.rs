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
enum FocusHorizon {
    Left(usize),
    Right(usize),
    Neutral,
}
impl Default for FocusHorizon {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Clone, Copy)]
enum FocusVert {
    Up(usize),
    Down(usize),
    Neutral,
}
impl Default for FocusVert {
    fn default() -> Self {
        Self::Neutral
    }
}

struct Game {
    cells: Box<[Box<[Cell]>]>,
    gen_frames: Option<u8>,
    focus_frames: Option<u8>,
    focus: (usize, usize),
    joystick: (FocusHorizon, FocusVert),
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
            joystick: Default::default(),
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

    fn move_focus(&mut self, hori: FocusHorizon, vert: FocusVert) {
        use FocusHorizon::*;
        use FocusVert::*;
        let (w, h) = (self.cells[0].len(), self.cells.len());
        let (x, y) = self.focus;
        let x = match hori {
            Left(dx) if x >= dx => x - dx,
            Right(dx) if w > x + dx => x + dx,
            _ => x,
        };
        let y = match vert {
            Up(dy) if y >= dy => y - dy,
            Down(dy) if h > y + dy => y + dy,
            _ => y,
        };
        self.focus = (x, y);
        self.focus_frames = Some(FOCUS_FRAMES);
    }

    fn stick_horizontal(&mut self, h: FocusHorizon) {
        self.joystick.0 = h;
    }
    fn stick_vertical(&mut self, v: FocusVert) {
        self.joystick.1 = v;
    }

    fn flip_cell(&mut self, x_px: usize, y_px: usize) {
        let stride = CELL_PX as usize + 1;
        let (x, y) = (x_px / stride, y_px / stride);
        self.cells[y][x].flip();
    }

    fn flip_cell_at_focus(&mut self) {
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
        match self.joystick {
            (FocusHorizon::Neutral, FocusVert::Neutral) => {}
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

    fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
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
                } => game.flip_cell(x as usize, y as usize),
                MouseButtonDown {
                    mouse_btn: MouseButton::Right,
                    ..
                } => game.toggle(),
                MouseMotion { x, y, .. } => game.focus(x as usize, y as usize),
                KeyDown {
                    keycode: Some(Keycode::Down | Keycode::J),
                    ..
                } => game.move_focus(FocusHorizon::Neutral, FocusVert::Down(1)),
                KeyDown {
                    keycode: Some(Keycode::Up | Keycode::K),
                    ..
                } => game.move_focus(FocusHorizon::Neutral, FocusVert::Up(1)),
                KeyDown {
                    keycode: Some(Keycode::Left | Keycode::H),
                    ..
                } => game.move_focus(FocusHorizon::Left(1), FocusVert::Neutral),
                KeyDown {
                    keycode: Some(Keycode::Right | Keycode::L),
                    ..
                } => game.move_focus(FocusHorizon::Right(1), FocusVert::Neutral),
                KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => game.flip_cell_at_focus(),
                KeyDown {
                    keycode: Some(Keycode::Return | Keycode::Return2),
                    ..
                } => game.toggle(),
                KeyDown {
                    keycode: Some(Keycode::Escape | Keycode::N),
                    ..
                } => game.reset(),
                KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => game.random(),
                JoyHatMotion {
                    which: 0,
                    state: HatState::Up,
                    ..
                } => game.move_focus(FocusHorizon::Neutral, FocusVert::Up(1)),
                JoyHatMotion {
                    which: 0,
                    state: HatState::RightUp,
                    ..
                } => game.move_focus(FocusHorizon::Right(1), FocusVert::Up(1)),
                JoyHatMotion {
                    which: 0,
                    state: HatState::Right,
                    ..
                } => game.move_focus(FocusHorizon::Right(1), FocusVert::Neutral),
                JoyHatMotion {
                    which: 0,
                    state: HatState::RightDown,
                    ..
                } => game.move_focus(FocusHorizon::Right(1), FocusVert::Down(1)),
                JoyHatMotion {
                    which: 0,
                    state: HatState::Down,
                    ..
                } => game.move_focus(FocusHorizon::Neutral, FocusVert::Down(1)),
                JoyHatMotion {
                    which: 0,
                    state: HatState::LeftDown,
                    ..
                } => game.move_focus(FocusHorizon::Left(1), FocusVert::Down(1)),
                JoyHatMotion {
                    which: 0,
                    state: HatState::Left,
                    ..
                } => game.move_focus(FocusHorizon::Left(1), FocusVert::Neutral),
                JoyHatMotion {
                    which: 0,
                    state: HatState::LeftUp,
                    ..
                } => game.move_focus(FocusHorizon::Left(1), FocusVert::Up(1)),
                JoyHatMotion {
                    which: 0,
                    state: HatState::Centered,
                    ..
                } => game.move_focus(FocusHorizon::Neutral, FocusVert::Neutral),
                JoyButtonDown {
                    which: 0,
                    button_idx: 0,
                    ..
                } => game.flip_cell_at_focus(), // Cross button of DualShock4
                JoyButtonDown {
                    which: 0,
                    button_idx: 1,
                    ..
                } => game.toggle(), // Circle button of DualShock4
                JoyButtonDown {
                    which: 0,
                    button_idx: 2,
                    ..
                } => game.random(), // Square button of DualShock4
                JoyButtonDown {
                    which: 0,
                    button_idx: 3,
                    ..
                } => game.reset(), // Triangle button of DualShock4
                JoyAxisMotion {
                    which: 0,
                    axis_idx: 0,
                    value: -9999..=9999,
                    ..
                } => game.stick_horizontal(FocusHorizon::Neutral),
                JoyAxisMotion {
                    which: 0,
                    axis_idx: 0,
                    value: -29999..=-10000,
                    ..
                } => game.stick_horizontal(FocusHorizon::Left(1)),
                JoyAxisMotion {
                    which: 0,
                    axis_idx: 0,
                    value: -32768..=-30000,
                    ..
                } => game.stick_horizontal(FocusHorizon::Left(2)),
                JoyAxisMotion {
                    which: 0,
                    axis_idx: 0,
                    value: 10000..=29999,
                    ..
                } => game.stick_horizontal(FocusHorizon::Right(1)),
                JoyAxisMotion {
                    which: 0,
                    axis_idx: 0,
                    value: 30000..=32767,
                    ..
                } => game.stick_horizontal(FocusHorizon::Right(2)),
                JoyAxisMotion {
                    which: 0,
                    axis_idx: 1,
                    value: -9999..=9999,
                    ..
                } => game.stick_vertical(FocusVert::Neutral),
                JoyAxisMotion {
                    which: 0,
                    axis_idx: 1,
                    value: -29999..=-10000,
                    ..
                } => game.stick_vertical(FocusVert::Up(1)),
                JoyAxisMotion {
                    which: 0,
                    axis_idx: 1,
                    value: -32768..=-30000,
                    ..
                } => game.stick_vertical(FocusVert::Up(2)),
                JoyAxisMotion {
                    which: 0,
                    axis_idx: 1,
                    value: 10000..=29999,
                    ..
                } => game.stick_vertical(FocusVert::Down(1)),
                JoyAxisMotion {
                    which: 0,
                    axis_idx: 1,
                    value: 30000..=32767,
                    ..
                } => game.stick_vertical(FocusVert::Down(2)),
                JoyDeviceAdded { which: 0, .. } if controller.is_none() => {
                    controller = Some(joystick.open(0).map_err(|e| format!("{}", e))?);
                }
                _ => {}
            }
        }
        game.update();
        game.draw(&mut canvas)?;
        canvas.present();
        let now = Instant::now();
        let _fps = 1000.0 / now.duration_since(stamp).subsec_millis() as f64;
        // println!("{:?} fps", fps);
        stamp = now;
    }

    Ok(())
}
