use bytemuck::{Pod, Zeroable};
use std::f32;
use std::iter;
use std::mem;
use wgpu::util::DeviceExt as _;
use winit::dpi::PhysicalSize;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use winit::window::WindowBuilder;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        //         │◄────── array_stride ───────►│◄───── array_stride ────────►│
        //         │◄─ position ─►│◄── color ───►│◄─ position ─►│◄── color ───►│
        //         ┌──────────────┬──────────────┬──────────────┬──────────────┬─
        // buffer: │ Float32x3    │ Float32x3    │ Float32x3    │ Float32x3    │ ...
        //         └──────────────┴──────────────┴──────────────┴──────────────┴─
        // offset: 0            32x3          32x6           32x9          32x12
        const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // `position` field in `Vertex`
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // `color` field in `Vertex`
                wgpu::VertexAttribute {
                    offset: wgpu::VertexFormat::Float32x3.size(),
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        };
        LAYOUT
    }
}

#[derive(Debug)]
struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    // Config and size are necessary for reconfiguring surface on window reisize
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    bg_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertices: [Vertex; 3],
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

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle_shader.wgsl").into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false, // Related to anti-aliasing
            },
            multiview: None,
        });

        let bg_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        let vertices = [
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            bg_color,
            render_pipeline,
            vertex_buffer,
            vertices,
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
                self.bg_color = wgpu::Color { r, g, b, a };
                log::info!("Mouse {:?}: {:?}", position, self.bg_color);
                true
            }
            _ => false,
        }
    }

    fn update(&mut self) {
        // Update color
        let mut red = self.vertices[0].color;
        if red[2] >= 1.0 {
            let i = red.iter().position(|f| *f > -1.0).unwrap();
            red[i] -= 0.02;
        } else {
            let i = red.iter().position(|f| *f < 1.0).unwrap();
            red[i] += 0.02;
        }
        self.vertices[0].color = red;

        for i in 1..=2 {
            let mut c = red;
            c.rotate_right(i);
            self.vertices[i].color = c;
        }

        // Update position
        for i in 0..=2 {
            let (sin, cos) = (f32::consts::PI / 360.0).sin_cos();
            let [x, y, z] = self.vertices[i].position;
            let x = x * cos - y * sin;
            let y = x * sin + y * cos;
            self.vertices[i].position = [x, y, z];
        }

        self.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[
                // [[location(0)]] in the fragment shader targets this attachment
                wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.bg_color),
                        store: true,
                    },
                },
            ],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        // 0 means the first vertex buffer. Note that multiple vertex buffers can be declared in render pipeline
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        // Render all vertices. The indices are passed to [[builtin(vertex_index)]] in the vertex shader
        render_pass.draw(0..self.vertices.len() as u32, 0..1);

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
        Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
            if !state.input(event) {
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
            }
        }
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
