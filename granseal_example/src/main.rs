use std::collections::HashMap;
use rand::Rng;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use granseal_engine::{GransealGameConfig, run};
use granseal_engine::GransealGameState;
use granseal_engine::shape::*;

pub struct Vector2d {
    x: f32,
    y: f32,
}

impl Vector2d {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }
}

pub struct Entity {
    pos: Vector2d,
    size: Vector2d,
    velocity: Vector2d,
    color: Color,
}

impl Entity {
    fn random(w: f32, h: f32) -> Self {
        let mut r = rand::thread_rng();
        Self {
            pos: Vector2d::new(r.gen::<f32>() * w,r.gen::<f32>() * h),
            size: Vector2d::new(r.gen_range(16.0..64.00), r.gen_range(16.0..64.00)),
            velocity: Vector2d::new( r.gen_range(-2.0..2.0), r.gen_range(-2.0..2.0)),
            color: Color::rgb(r.gen(),r.gen(),r.gen()),
        }
    }
}

pub struct GameState {
    config: GransealGameConfig,
    position: Vector2d,
    init: bool,
    entities: Vec<Entity>,
}

impl GameState {
    fn new() -> Self {
        let mut entities = vec![];

        let mut r = rand::thread_rng();

        for x in (0..800).step_by(4) {
            for y in (0..600).step_by(4) {
                entities.push(Entity {
                    pos: Vector2d::new(x as f32,y as f32),
                    velocity: Vector2d::new(r.gen_range(-0.1..0.1),r.gen_range(-0.1..0.1)),
                    size: Vector2d::new(4.0,4.0),
                    color: Color::rgb(r.gen(),r.gen(),r.gen()),
                })
            }
        }
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
            init: false,
            entities,
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

        for mut e in &mut self.entities {
            e.pos.x += e.velocity.x;
            e.pos.y += e.velocity.y;
            if e.pos.x <= 0.0 {e.velocity.x *= -1.0}
            if e.pos.y <= 0.0 {e.velocity.y *= -1.0}
            if e.pos.x >= self.config.width as f32 - e.size.x {e.velocity.x *= -1.0}
            if e.pos.y >= self.config.height as f32 - e.size.y {e.velocity.y *= -1.0}
        }

    }

    fn render(&mut self, shapes: &mut Vec<Shape>) {
        let width = self.config.width as f32;
        let height = self.config.height as f32;

        shapes.clear();

        let mut r = rand::thread_rng();
        for e in &mut self.entities {
            e.color = Color::rgb(r.gen(),r.gen(),r.gen());
            shapes.push(Shape::rect(e.pos.x,e.pos.y,e.size.x,e.size.y).color(e.color));
        }

    }
}

fn main() {
    pollster::block_on(run(Box::new(GameState::new())));
}