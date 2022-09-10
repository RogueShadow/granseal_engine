// No console,
// #![windows_subsystem ="windows"]

use std::{
    collections::HashMap,
    ops::Index,
    time::Duration,
};
use rand_xorshift::XorShiftRng;
use rand::prelude::*;
use granseal_engine::{GransealGameConfig, events::{Event, Key}, GransealGameState, shape::*, VSyncMode};

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
    kind: ShapeKind,
}

impl Entity {
    fn random(w: f32, h: f32) -> Self {
        let mut r = XorShiftRng::from_rng(rand::thread_rng()).unwrap();
        let speed = 100.0;
        Self {
            pos: Vector2d::new(r.gen::<f32>() * w,r.gen::<f32>() * h),
            size: Vector2d::new(r.gen_range(16.0..64.00), r.gen_range(16.0..64.00)),
            velocity: Vector2d::new( r.gen_range(-speed..speed), r.gen_range(-speed..speed)),
            color: Color::rgb(r.gen(),r.gen(),r.gen()),
            kind: r.gen_range(0..6),
        }
    }
}

pub struct GameState {
    config: GransealGameConfig,
    position: Vector2d,
    entities: Vec<Entity>,
    rng: XorShiftRng,
}

impl GameState {
    fn new() -> Self {
        let mut entities = vec![];
        let mut r = XorShiftRng::from_rng(rand::thread_rng()).unwrap();

        for _i in 0..50 {
            entities.push(Entity::random(r.gen::<f32>() * 800.0,r.gen::<f32>() * 600.0));
        }

        // let step = 8;
        // let speed = 100.0;
        // for x in (0..800).step_by(step) {
        //     for y in (0..600).step_by(step) {
        //         entities.push(Entity {
        //             pos: Vector2d::new(x as f32,y as f32),
        //             //velocity: Vector2d::new(r.gen_range(-speed..speed),r.gen_range(-speed..speed)),
        //             velocity: Vector2d::new(0.0,0.0),
        //             size: Vector2d::new(step as f32, step as f32),
        //             color: Color::rgb(r.gen(),r.gen(),r.gen()),
        //             kind: RECT
        //         })
        //     }
        // }
        println!("Entities: {:?}",entities.len());
        Self {
            config: GransealGameConfig::new()
                .title("Shapes Go Fly WOooo")
                .vsync(VSyncMode::VSyncOff),
            position: Vector2d {
                x: 0.0,
                y: 0.0,
            },
            entities,
            rng: XorShiftRng::from_rng(rand::thread_rng()).unwrap(),
        }
    }
}


impl GransealGameState for GameState {
    fn config(&mut self) -> &mut GransealGameConfig {
        &mut self.config
    }
    fn event(&mut self, _event: &Event) -> bool {
        false
    }

    fn update(&mut self,delta: Duration, key_down: &HashMap<Key,bool>) {
        use Key::*;
        let key = |k: Key| -> bool {
            if key_down.contains_key(&k) {
                *key_down.index(&k)
            } else {false}
        };

        let speed = 250.0 * delta.as_secs_f32();
        if key(W) {self.position.y -= speed}
        if key(A) {self.position.x -= speed}
        if key(S) {self.position.y += speed}
        if key(D) {self.position.x += speed}

        for mut e in &mut self.entities {
            e.pos.x += e.velocity.x * delta.as_secs_f32();
            e.pos.y += e.velocity.y * delta.as_secs_f32();
            if e.pos.x <= 0.0 {e.velocity.x *= -1.0}
            if e.pos.y <= 0.0 {e.velocity.y *= -1.0}
            if e.pos.x >= self.config.width as f32 - e.size.x {e.velocity.x *= -1.0}
            if e.pos.y >= self.config.height as f32 - e.size.y {e.velocity.y *= -1.0}
        }

    }

    fn render(&mut self, g: &mut Graphics) {
        g.clear();
        g.set_translation(self.position.x,self.position.y);
        let r = &mut self.rng;
        for e in &mut self.entities {
            e.color = Color::rgb(r.gen(),r.gen(),r.gen());
            g.color(e.color);
            g.shape(
                r.gen_range(0..=5),
                e.pos.x,
                e.pos.y,
                e.size.x,
                e.size.y
            );
        }

    }
}

fn main() {
    granseal_engine::start(GameState::new());
}