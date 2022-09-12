use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};
use image::EncodableLayout;
use wgpu::util::DeviceExt;
use winit::event::WindowEvent;
use winit::window::Window;
use crate::{events, GransealGameState, Graphics, KeyState, map_events, map_present_modes, Shape, Texture, TextureInfo};

#[allow(unused)]
pub struct StateShapeRender {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    mouse_pos: [f64; 2],
    key_down: HashMap<events::Key,bool>,
    game_state: Box<dyn GransealGameState>,
    textures: HashMap<String,TextureInfo>,
    graphics: Graphics,
    shape_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    screen_buffer: wgpu::Buffer,
    screen_bind_group: wgpu::BindGroup,
    clear_color: [f64; 4],
    texture_bind_group_layout: wgpu::BindGroupLayout,
    timer: Instant,
    time_buffer: wgpu::Buffer,
}

impl StateShapeRender {
    pub(crate) async fn new(window: &Window, mut game_state: Box<dyn GransealGameState>) -> Self {
        let timer = std::time::Instant::now();
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
            present_mode: map_present_modes(game_state.config().vsync),
        };
        surface.configure(&device, &config);

        let mouse_pos = [0.0,0.0];

        let key_down = HashMap::new();

        let graphics = Graphics::new();

        let shape_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Shape Buffer"),
                contents: bytemuck::cast_slice(graphics.shapes.as_slice()),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let clear_color = [0.1,0.2,0.3,1.0];

        let screen_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Screen Buffer"),
                contents: bytemuck::cast_slice([config.width as f32,config.height as f32].as_bytes()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let time_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Time Buffer"),
                contents: bytemuck::cast_slice(timer.elapsed().as_secs_f32().to_ne_bytes().as_slice()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let screen_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count:  None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
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
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: time_buffer.as_entire_binding(),
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
                cull_mode: None,
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

        StateShapeRender {
            surface,
            device,
            queue,
            config,
            size,
            mouse_pos,
            key_down,
            game_state,
            textures: HashMap::new(),
            graphics,
            shape_buffer,
            render_pipeline,
            screen_buffer,
            screen_bind_group,
            clear_color,
            texture_bind_group_layout,
            timer,
            time_buffer,
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
            &self.texture_bind_group_layout,
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

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device,&self.config);
            self.queue.write_buffer(&self.screen_buffer, 0, bytemuck::cast_slice([new_size.width as f32,new_size.height as f32].as_bytes()));
        }
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
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

    pub(crate) fn update(&mut self, delta_time: Duration) {
        self.game_state.update(delta_time, &self.key_down);
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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

        self.queue.write_buffer(&self.time_buffer, 0, &self.timer.elapsed().as_secs_f32().to_ne_bytes().as_slice());

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
                            r: *self.clear_color.get(0).unwrap(),
                            g: *self.clear_color.get(1).unwrap(),
                            b: *self.clear_color.get(2).unwrap(),
                            a: *self.clear_color.get(3).unwrap(),
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None
            });

            self.load("happy-tree.png");
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0,&self.screen_bind_group,&[]);
            render_pass.set_bind_group(1, &self.textures.get("happy-tree.png").unwrap().bind_group, &[]);
            render_pass.set_vertex_buffer(0,self.shape_buffer.slice(..));
            render_pass.draw(0..5 as u32, 0..self.graphics.shapes.len() as u32);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}