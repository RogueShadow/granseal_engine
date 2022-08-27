use std::collections::HashMap;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use granseal_engine::{GransealGameConfig, run};
use granseal_engine::GransealGameState;
use granseal_engine::shape::*;

pub struct Vector2d {
    x: f32,
    y: f32,
}

pub struct GameState {
    config: GransealGameConfig,
    position: Vector2d,
}

impl GameState {
    fn new() -> Self {
        Self {
            config: GransealGameConfig {
                title: "Granseal WGPU Experimental Shapes",
                width: 800,
                height: 600,
            },
            position: Vector2d {
                x: 0.0,
                y: 0.0,
            },
        }
    }
}

impl GransealGameState for GameState {
    fn config(&mut self) -> &GransealGameConfig {
        &self.config
    }
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self, key_down: &HashMap<VirtualKeyCode,bool>) {
        use VirtualKeyCode::*;
        let key = |k: VirtualKeyCode| -> bool {
            *key_down.get(&k).unwrap_or(&false)
        };

        let speed = 0.75;
        if key(W) {self.position.y -= speed}
        if key(A) {self.position.x -= speed}
        if key(S) {self.position.y += speed}
        if key(D) {self.position.x += speed}

    }

    fn render(&mut self, shapes: &mut Vec<Shape>) {
        let width = self.config.width as f32;
        let height = self.config.height as f32;
        shapes.clear();
        shapes.push(Shape::rect(0.0,height - 32.0,width,32.0).color(BLACK));
        shapes.push(Shape::square(self.position.x,self.position.y,64.0).color(BLUE));
    }
}

fn main() {
    pollster::block_on(run(Box::new(GameState::new())));
}