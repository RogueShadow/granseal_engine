
use std::collections::HashMap;
use std::ops::Index;
use std::rc::Rc;
use std::time::{Duration, Instant};
use image::EncodableLayout;
use wgpu::{BufferAddress, Color, Extent3d, TextureDimension, TextureFormat, TextureUsages};
use wgpu::util::DeviceExt;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::{events, GransealGameConfig, GransealGameState, Graphics, KeyState, map_events, map_present_modes, Shape, Texture, TextureInfo};
use crate::events::Event;

#[derive(Copy,Clone,Debug)]
pub enum GransealError {
    EventError,
    AdapterErr,
    DeviceErr,
    FormatErr,
}

pub struct Castle {
    pub key_down: HashMap<events::Key,bool>,
    mouse_pos: [f64; 2],
    clear_color: [f64; 4],
    timer: Instant,
    pub clear: bool,
}

impl Castle {
    pub fn key(&self,k: events::Key) -> bool {
        if self.key_down.contains_key(&k) {
            *self.key_down.index(&k)
        } else {false}
    }
    pub fn clear(&mut self,value: bool) {
        self.clear = value;
    }
}


#[allow(unused)]
pub struct GransealEngine {
    pub(crate) window: winit::window::Window,
    pub engine_cfg: GransealGameConfig,
    surface: wgpu::Surface,
    device: std::rc::Rc<wgpu::Device>,
    queue: std::rc::Rc<wgpu::Queue>,
    pub(crate) surface_cfg: wgpu::SurfaceConfiguration,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    pub(crate) game_state: Box<dyn GransealGameState>,
    graphics: Graphics,
    shape_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    screen_buffer: wgpu::Buffer,
    screen_bind_group: wgpu::BindGroup,
    time_buffer: wgpu::Buffer,
    castle: Castle,
}

impl GransealEngine {
    pub(crate) async fn new(window: Window,engine_cfg: GransealGameConfig, mut game_state: Box<dyn GransealGameState>) -> Result<GransealEngine,GransealError> {
        let timer = std::time::Instant::now();
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.ok_or(GransealError::AdapterErr)?;

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty() ,
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.ok().ok_or(GransealError::DeviceErr)?;
        let device = Rc::new(device);
        let queue = Rc::new(queue);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter).pop().ok_or(GransealError::FormatErr)?,
            width: size.width,
            height: size.height,
            present_mode: map_present_modes(engine_cfg.vsync),
        };
        surface.configure(&device, &config);

        let mouse_pos = [0.0,0.0];

        let key_down = HashMap::new();

        let graphics = Graphics::new(device.clone(),queue.clone());

        let shape_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Shape Buffer"),
                contents: bytemuck::cast_slice(graphics.shapes.as_slice()),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let clear_color = engine_cfg.clear_color;

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
                },
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
                },
            ],
            label: Some("screen_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shape_shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Shape Render Pipeline Layout"),
                bind_group_layouts: &[&screen_bind_group_layout,&graphics.texture_bind_group_layout],
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
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let castle = Castle {
            key_down,
            mouse_pos,
            clear_color,
            timer,
            clear: true,
        };

        Ok(GransealEngine {
            window,
            engine_cfg,
            surface,
            device,
            queue,
            surface_cfg: config,
            size,
            game_state,
            graphics,
            shape_buffer,
            render_pipeline,
            screen_buffer,
            screen_bind_group,
            time_buffer,
            castle,
        })
    }



    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.surface_cfg.width = new_size.width;
            self.surface_cfg.height = new_size.height;
            self.surface.configure(&self.device,&self.surface_cfg);
            self.queue.write_buffer(&self.screen_buffer, 0, bytemuck::cast_slice([new_size.width as f32,new_size.height as f32].as_bytes()));
        }
        self.event(Event::Resized(new_size.width,new_size.height));
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> Result<bool,GransealError> {
        let granseal_event = map_events(event);
        if granseal_event.is_some() {
            match granseal_event.ok_or(GransealError::EventError)? {
                events::Event::KeyEvent {
                    state, key, modifiers: _modifiers
                } => {
                    match state {
                        KeyState::Pressed => {
                            self.castle.key_down.insert(key,true);
                        },
                        KeyState::Released => {
                            self.castle.key_down.insert(key,false);
                        }
                    }
                }
                _ => {}
            }
            if self.event(granseal_event.ok_or(GransealError::EventError)?) {
                return Ok(true);
            }
        }
        return Ok(false);
    }

    pub(crate) fn update(&mut self, delta_time: Duration) {
        self.event(Event::Update(delta_time));
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.event(Event::Draw);
        let shape_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Shape Buffer"),
                contents: bytemuck::cast_slice(self.graphics.shapes.as_slice()),
                usage: wgpu::BufferUsages::VERTEX
            }
        );
        self.shape_buffer.destroy();
        self.shape_buffer = shape_buffer;

        self.queue.write_buffer(&self.time_buffer, 0, &self.castle.timer.elapsed().as_secs_f32().to_ne_bytes().as_slice());

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
                        load: if self.castle.clear {wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.castle.clear_color[0],
                            g: self.castle.clear_color[1],
                            b: self.castle.clear_color[2],
                            a: self.castle.clear_color[3],
                        })} else  {wgpu::LoadOp::Load},
                        store: true,
                    },
                })],
                depth_stencil_attachment: None
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0,self.shape_buffer.slice(..));
            render_pass.set_bind_group(0,&self.screen_bind_group,&[]);


            for (i, _) in self.graphics.shapes.iter_mut().enumerate() {
                let tex = if self.graphics.images.contains_key(&i) {
                    self.graphics.images.index(&i)
                } else {Graphics::ERROR_IMG};
                let t = self.graphics.textures.get(tex);
                match t {
                    Some(x) => {
                        render_pass.set_bind_group(1,&x.bind_group, &[]);
                    },
                    None => {
                        let path = std::env::current_dir().expect("Couldn't get the current directory.");
                        println!("Couldn't find texture: {} in path: {:?}",tex,path);
                    },
                }
                render_pass.draw(0..5 as u32,i as u32..(i+1) as u32);
            }
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
    pub fn event(&mut self, e: Event) -> bool {
        self.game_state.event(&mut self.graphics,&mut self.castle,&e)
    }
}