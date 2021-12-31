use std::iter;
use winit::dpi::PhysicalSize;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use winit::window::WindowBuilder;

#[derive(Debug)]
struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    // Config and size are necessary for reconfiguring surface on window reisize
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    color: wgpu::Color,
}

impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        assert!(size.height > 0 && size.width > 0);

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        Self {
            surface,
            device,
            queue,
            config,
            size,
            color,
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            log::info!("Resized window: {:?}", new_size);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let (w, h) = (self.size.width as f64, self.size.height as f64);
                let (x, y) = (position.x, position.y);
                let max = (w * w + h * h).sqrt();
                let r = (x * x + y * y).sqrt() / max;
                let g = ((w - x) * (w - x) + y * y).sqrt() / max;
                let b = (x * x + (h - y) * (h - y)).sqrt() / max;
                let a = 1.0;
                self.color = wgpu::Color { r, g, b, a };
                log::info!("Mouse {:?}: {:?}", position, self.color);
                true
            }
            _ => false,
        }
    }

    fn update(&mut self) {
        // Do nothing for now
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.color),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        drop(render_pass); // render_pass borrows encoder mutably. It must be dropped before calling finish().

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = pollster::block_on(State::new(&window));
    log::info!("{:?}", state);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => if !state.input(event) {
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => state.resize(*size),
                WindowEvent::ScaleFactorChanged {
                    new_inner_size,
                    scale_factor: _,
                } => state.resize(**new_inner_size),
                _ => {}
            }
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size), // Reconfigure
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => log::error!("Error while rendering to surface: {:?}", e), // Other errors (outdated, timeout) should be resolved by the next frame
            }
        }
        Event::MainEventsCleared => window.request_redraw(),
        _ => {}
    });
}
