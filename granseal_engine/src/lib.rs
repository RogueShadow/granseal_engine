use std::{
    collections::HashMap,
    time::Duration,
};
use std::path::Path;
use cgmath::num_traits::ToPrimitive;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use wgpu::{
    util::DeviceExt,
};

use winit::{
    event::*,
    event_loop::{
        ControlFlow,
        EventLoop
    },
    window::{
        WindowBuilder,
        Window
    },
};

use crate::events::{KeyState, map_events};
use crate::shape::*;
use crate::texture::{Texture, TextureInfo};

mod texture;
pub mod shape;
pub mod events;

struct ShapePipeline {
    render_pipeline: wgpu::RenderPipeline,
    screen_uniform: ScreenUniform,
    screen_buffer: wgpu::Buffer,
    screen_bind_group: wgpu::BindGroup,
    clear_color: [f64; 4],
    texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl ShapePipeline {
    fn new(config: &wgpu::SurfaceConfiguration, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let clear_color = [0.1,0.2,0.3,1.0];
        let screen_uniform = ScreenUniform::new(config.width as f32,config.height as f32);
        let screen_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Screen Buffer"),
                contents: bytemuck::cast_slice(&[screen_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let screen_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count:  None,
                }
            ],
            label: Some("screen_bind_group_layout"),
        });
        let screen_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &screen_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: screen_buffer.as_entire_binding(),
                }
            ],
            label: Some("screen_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shape_shader.wgsl").into()),
        });

        let texture_bind_group_layout = device.create_bind_group_layout( &wgpu::BindGroupLayoutDescriptor {
            label: Some("texture_bind_group_layout"),
            entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false
                },
                count: None
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None
            }
            ]
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Shape Render Pipeline Layout"),
                bind_group_layouts: &[&screen_bind_group_layout,&texture_bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shape Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Shape::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
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
                alpha_to_coverage_enabled: true,
            },
            multiview: None,
        });

        Self {
            render_pipeline,
            screen_uniform,
            screen_buffer,
            screen_bind_group,
            clear_color,
            texture_bind_group_layout,
        }
    }
}

#[allow(unused)]
struct StateShapeRender {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pipeline: ShapePipeline,
    mouse_pos: [f64; 2],
    key_down: HashMap<events::Key,bool>,
    game_state: Box<dyn GransealGameState>,
    textures: HashMap<String,TextureInfo>,
    graphics: Graphics,
    shape_buffer: wgpu::Buffer,
}

impl StateShapeRender {
    async fn new(window: &Window, game_state: Box<dyn GransealGameState>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter).pop().unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        surface.configure(&device, &config);

        let mouse_pos = [0.0,0.0];

        let pipeline = ShapePipeline::new(&config,&device,&queue);

        let key_down = HashMap::new();

        let graphics = Graphics::new();

        let shape_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Shape Buffer"),
                contents: bytemuck::cast_slice(graphics.shapes.as_slice()),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        StateShapeRender {
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
            mouse_pos,
            key_down,
            game_state,
            textures: HashMap::new(),
            graphics,
            shape_buffer,
        }
    }

    fn load<P>(&mut self, image: P) where P: AsRef<Path> {
        let path = image.as_ref().clone().to_str().unwrap();
        if self.textures.contains_key(path) {
            return;
        }

        let img = &image::open(&image).unwrap();
        let texture = Texture::from_image(
            &self.device,
            &self.queue,
            img,
            Some(path),
            &self.pipeline.texture_bind_group_layout,
        ).unwrap();
        let texture_info = TextureInfo {
            bind_group: texture.bind_group,
            path: path.to_string(),
            alias: Some(path.to_string()),
            width: img.width(),
            height: img.height(),
        };

        self.textures.insert(path.to_string(),texture_info);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device,&self.config);
            self.pipeline.screen_uniform.update(new_size.width as f32,new_size.height as f32);
            self.queue.write_buffer(&self.pipeline.screen_buffer, 0, bytemuck::cast_slice(&[self.pipeline.screen_uniform]));
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        let granseal_event = map_events(event);
        if granseal_event.is_some() {
            match granseal_event.unwrap() {
                events::Event::KeyEvent {
                    state, key, modifiers: _modifiers
                } => {
                    match state {
                        KeyState::Pressed => {
                            self.key_down.insert(key,true);
                        },
                        KeyState::Released => {
                            self.key_down.insert(key,false);
                        }
                    }
                }
                _ => {}
            }
            if self.game_state.event(&granseal_event.unwrap()) {
                return true;
            }
        }
        return false;
    }

    fn update(&mut self, delta_time: Duration) {
        self.game_state.update(delta_time, &self.key_down);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.game_state.render(&mut self.graphics);
        let shape_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Shape Buffer"),
                contents: bytemuck::cast_slice(self.graphics.shapes.as_slice()),
                usage: wgpu::BufferUsages::VERTEX
            }
        );
        self.shape_buffer.destroy();
        self.shape_buffer = shape_buffer;

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view:  &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: *self.pipeline.clear_color.get(0).unwrap(),
                            g: *self.pipeline.clear_color.get(1).unwrap(),
                            b: *self.pipeline.clear_color.get(2).unwrap(),
                            a: *self.pipeline.clear_color.get(3).unwrap(),
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None
            });

            self.load("happy-tree.png");
            render_pass.set_pipeline(&self.pipeline.render_pipeline);
            render_pass.set_bind_group(0,&self.pipeline.screen_bind_group,&[]);
            render_pass.set_bind_group(1, &self.textures.get("happy-tree.png").unwrap().bind_group, &[]);
            render_pass.set_vertex_buffer(0,self.shape_buffer.slice(..));
            render_pass.draw(0..5 as u32, 0..self.graphics.shapes.len() as u32);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ScreenUniform {
    width: f32,
    height: f32,
}

impl ScreenUniform {
    fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }
    fn update(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

#[derive(Clone,Copy)]
pub struct GransealGameConfig {
    pub width: i32,
    pub height: i32,
    pub title: &'static str,
}

pub trait GransealGameState {
    fn config(&mut self) -> &GransealGameConfig;
    fn event(&mut self, event: &events::Event) -> bool;
    fn update(&mut self,delta: Duration, key_down: &HashMap<events::Key,bool>);
    fn render(&mut self, graphics: &mut Graphics);
}

pub fn start(state: Box<dyn GransealGameState>) {
    pollster::block_on(run(state));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run(mut game_state: Box<dyn GransealGameState>) {


    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let config = game_state.config().clone();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(config.title)
        .with_inner_size(winit::dpi::PhysicalSize {
            width: config.width,
            height: config.height,
        })
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(450, 400));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

    let mut state = StateShapeRender::new(&window, game_state).await;
    let mut frames = 0;
    let mut timer = std::time::Instant::now();
    let mut delta = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {

        match event {
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
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, ..} => {
                        state.resize( **new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update(delta.elapsed());
                delta = std::time::Instant::now();
                match state.render() {
                    Ok(_) => {
                        frames += 1;
                        if timer.elapsed().as_secs_f64() > 1.0 {
                            window.set_title(format!("{}: {}",config.title,frames).as_str());
                            frames = 0;
                            timer = std::time::Instant::now();
                        }
                    }
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}",e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}