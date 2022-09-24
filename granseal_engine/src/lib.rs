extern crate core;
use std::{
    time::Duration,
};
use winit::{
    event::*,
    event_loop::{
        ControlFlow,
        EventLoop
    },
    window::{
        WindowBuilder,
    },
};

use crate::events::{KeyState, map_events};
use crate::renderer::{Castle, StateShapeRender};
use crate::shape::*;
use crate::texture::{Texture, TextureInfo};

mod texture;
pub mod shape;
pub mod events;
pub mod renderer;


#[repr(C)]
#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub enum VSyncMode {
    AutoVsync,
    AutoNoVsync,
    VSyncOn,
    AdaptiveVSync,
    VSyncOff,
    FastVSync,
}

fn map_present_modes(mode: VSyncMode) -> wgpu::PresentMode {
    match mode  {
        VSyncMode::AutoVsync => wgpu::PresentMode::AutoVsync,
        VSyncMode::AutoNoVsync => wgpu::PresentMode::AutoNoVsync,
        VSyncMode::VSyncOn => wgpu::PresentMode::Fifo,
        VSyncMode::AdaptiveVSync => wgpu::PresentMode::FifoRelaxed,
        VSyncMode::VSyncOff => wgpu::PresentMode::Immediate,
        VSyncMode::FastVSync => wgpu::PresentMode::Mailbox,
    }
}

#[derive(Clone,Debug)]
pub struct GransealGameConfig {
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub vsync: VSyncMode,
    pub clear_color: [f64;4],
}

impl GransealGameConfig {
    pub fn new() -> Self {
        Self {
            title: "Granseal Engine".to_string(),
            width: 800,
            height: 600,
            vsync: VSyncMode::VSyncOn,
            clear_color: [0.0,0.0,0.0,1.0],
        }
    }
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
    pub fn vsync(mut self, mode: VSyncMode) -> Self {
        self.vsync = mode;
        self
    }
    pub fn clear_color(mut self, color: [f64;4]) -> Self {
        self.clear_color = color;
        self
    }
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

pub trait GransealGameState {
    fn event(&mut self, event: &events::Event) -> bool;
    fn update(&mut self,delta: Duration, castle: &mut Castle);
    fn render(&mut self, graphics: &mut Graphics);
}

pub fn start<S>(state: S, config: GransealGameConfig) where S: GransealGameState + 'static {
    pollster::block_on(run(Box::new(state), config));
}

pub async fn run(mut game_state: Box<dyn GransealGameState>, config: GransealGameConfig) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(&config.title)
        .with_inner_size(winit::dpi::PhysicalSize {
            width: config.width,
            height: config.height,
        })
        .build(&event_loop)
        .expect("Error creating window.");

    let mut state = StateShapeRender::new(window, config,game_state).await.expect("Failed to initialize render engine.");
    let mut frames = 0;
    let mut frame_timer = std::time::Instant::now();
    let mut delta = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window.id() => if !state.input(event).expect("Error handling input events.") {
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
            Event::RedrawRequested(window_id) if window_id == state.window.id() => {
                state.update(delta.elapsed());
                delta = std::time::Instant::now();
                match state.render() {
                    Ok(_) => {
                        frames += 1;
                        if frame_timer.elapsed().as_secs_f64() > 1.0 {
                            state.window.set_title(format!("{}: {}", &state.engine_cfg.title, frames).as_str());
                            frames = 0;
                            frame_timer = std::time::Instant::now();
                        }
                    }
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}",e),
                }
            }
            Event::MainEventsCleared => {
                state.window.request_redraw();
            }
            _ => {}
        }
    });
}