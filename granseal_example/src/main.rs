// No console,
// #![windows_subsystem ="windows"]

use std::{
    time::Duration,
};

use rand::prelude::*;
use rand_xorshift::XorShiftRng;

use granseal_engine::{events::{Event, Key}, GransealGameConfig, GransealGameState, shape::*, VSyncMode};
use granseal_engine::events::KeyState;
use granseal_engine::renderer::{Castle};

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
    angle: f32,
    a_vel: f32,
    image: Option<String>,
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
            angle: r.gen_range(0.0..6.28),
            a_vel: r.gen_range(-6.0..6.0),
            image: None,
            kind: r.gen_range(0..=3),
        }
    }
    fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector2d::new(x,y),
            size: Vector2d::new(1.0,1.0),
            velocity: Vector2d::new(0.0,0.0),
            color: Color::WHITE,
            angle: 0.0,
            a_vel: 0.0,
            image: None,
            kind: FILL_RECT,
        }
    }
    fn size(mut self,w: f32, h: f32) -> Self {
        self.size = Vector2d::new(w,h);
        self
    }
    fn velocity(mut self, vx: f32, vy: f32) -> Self {
        self.velocity = Vector2d::new(vx,vy);
        self
    }
    fn color(mut self,color: Color) -> Self  {
        self.color = color;
        self
    }
    fn angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }
    fn kind(mut self, kind: ShapeKind) -> Self {
        self.kind = kind;
        self
    }
    fn image(mut self, img: String) -> Self {
        self.image = Some(img);
        self.kind = TEX_RECT;
        self
    }
}

pub struct GameState {
    timer: std::time::Instant,
    config: GransealGameConfig,
    position: Vector2d,
    entities: Vec<Entity>,
    rng: XorShiftRng,
    clear: bool,
    bounce: bool,
    flash: bool,
    rotate: bool,
    init: bool,
    clear_cache: bool,
}

impl GameState {
    fn new() -> Self {
        let mut entities = vec![];
        let mut r = XorShiftRng::from_rng(rand::thread_rng()).unwrap();

        for _i in 0..1_00 {
            entities.push(Entity::random(r.gen::<f32>() * 800.0,r.gen::<f32>() * 600.0));
        }

        let mut test = vec!(
            Entity::new(0.0,0.0).size(64.0,64.0).color(Color::NAVY).kind(FILL_RECT),
            Entity::new(800.0 - 64.0,0.0).size(64.0,64.0).color(Color::NAVY).kind(FILL_RECT),
            Entity::new(800.0 - 64.0,600.0 - 64.0).size(64.0,64.0).color(Color::NAVY).kind(FILL_RECT),
            Entity::new( 0.0, 600.0 - 64.0).size(64.0,64.0).color(Color::NAVY).kind(FILL_RECT),

            Entity::new(64.0,64.0).size(64.0,64.0).color(Color::CYAN).kind(RECT),
            Entity::new(800.0 - 64.0 - 64.0,64.0).size(64.0,64.0).color(Color::CYAN).kind(RECT),
            Entity::new(800.0 - 64.0 - 64.0,600.0 - 64.0 - 64.0).size(64.0,64.0).color(Color::CYAN).kind(RECT),
            Entity::new( 64.0, 600.0 - 64.0 - 64.0).size(64.0,64.0).color(Color::CYAN).kind(RECT),

            Entity::new(0.0,64.0).size(64.0,64.0).color(Color::WHITE).kind(TEX_RECT).image(String::from("blob.png")),
            Entity::new(800.0 - 64.0,64.0).size(64.0,64.0).color(Color::WHITE).kind(TEX_RECT),
            Entity::new(800.0 - 64.0,600.0 - 128.0).size(64.0,64.0).color(Color::WHITE).kind(TEX_RECT),
            Entity::new( 0.0, 600.0 - 128.0).size(64.0,64.0).color(Color::WHITE).kind(TEX_RECT),

            Entity::new(64.0,128.0).size(64.0,64.0).color(Color::WHITE).kind(TEX_OVAL),
            Entity::new(800.0 - 64.0 - 64.0,128.0).size(64.0,64.0).color(Color::WHITE).kind(TEX_OVAL),
            Entity::new(800.0 - 64.0 - 64.0,600.0 - 128.0 - 64.0).size(64.0,64.0).color(Color::WHITE).kind(TEX_OVAL),
            Entity::new( 64.0, 600.0 - 64.0 - 128.0).size(64.0,64.0).color(Color::WHITE).kind(TEX_OVAL),

            Entity::new(128.0,128.0).size(64.0,64.0).color(Color::MAGENTA).kind(OVAL),
            Entity::new(800.0 - 128.0 - 64.0,128.0).size(64.0,64.0).color(Color::MAGENTA).kind(OVAL),
            Entity::new(800.0 - 128.0 - 64.0,600.0 - 128.0 - 64.0).size(64.0,64.0).color(Color::MAGENTA).kind(OVAL),
            Entity::new( 128.0, 600.0 - 128.0 - 64.0).size(64.0,64.0).color(Color::MAGENTA).kind(OVAL),

            Entity::new(192.0,192.0).size(64.0,64.0).color(Color::RED).kind(FILL_OVAL),
            Entity::new(800.0 - 256.0,192.0).size(64.0,64.0).color(Color::RED).kind(FILL_OVAL),
            Entity::new(800.0 - 256.0,600.0 - 256.0).size(64.0,64.0).color(Color::RED).kind(FILL_OVAL),
            Entity::new( 192.0, 600.0 - 256.0).size(64.0,64.0).color(Color::RED).kind(FILL_OVAL),
        );

        entities.append(&mut test);

        // let step = 64;
        // let speed = 50.0;
        // for x in (0..800).step_by(step) {
        //     for y in (0..600).step_by(step) {
        //         entities.push(Entity {
        //             pos: Vector2d::new(x as f32,y as f32),
        //             velocity: Vector2d::new(r.gen_range(-speed..speed),r.gen_range(-speed..speed)),
        //             //velocity: Vector2d::new(0.0,0.0),
        //             size: Vector2d::new(step as f32, step as f32),
        //             color: Color::rgb(r.gen(),r.gen(),r.gen()),
        //             angle: 0.0,
        //             a_vel: r.gen_range(-6.0..6.0),
        //             kind: TEX_OVAL
        //         })
        //     }
        // }

        entities.iter_mut().for_each(|f|{f.a_vel= r.gen_range(-6.0..6.0)});
        println!("Entities: {:?}",entities.len());
        Self {
            timer: std::time::Instant::now(),
            config: GransealGameConfig::new()
                .title("Press '1' '2' '3' hold '4' 'F5' to reload images")
                .vsync(VSyncMode::VSyncOff)
                .clear_color([0.0,0.0,0.0,1.0]),
            position: Vector2d {
                x: 0.0,
                y: 0.0,
            },
            entities,
            rng: XorShiftRng::from_rng(rand::thread_rng()).unwrap(),
            clear: true,
            bounce: false,
            flash: false,
            rotate: false,
            init: false,
            clear_cache: false,
        }
    }
}

impl GransealGameState for GameState {
    fn config(&mut self) -> &mut GransealGameConfig {
        &mut self.config
    }
    fn event(&mut self, event: &Event) -> bool {
        match event {
            Event::KeyEvent {
                state: KeyState::Pressed,
                key: Key::Key1,
                ..
            } => {self.bounce = !self.bounce}
            Event::KeyEvent {
                state: KeyState::Pressed,
                key: Key::Key2,
                ..
            } => {self.flash = !self.flash}
            Event::KeyEvent {
                state: KeyState::Pressed,
                key: Key::Key3,
                ..
            } => {self.rotate = !self.rotate}
            Event::KeyEvent {
                state,
                key: Key::Key4,
                ..
            } => {self.clear = match state {
                KeyState::Pressed => {false}
                KeyState::Released => {true}
            }}
            Event::KeyEvent {
                state: KeyState::Pressed,
                key: Key::F5,
                ..
            } => {self.clear_cache = true}
            Event::MouseButton { .. } => {}
            Event::MouseMoved { .. } => {}
            _ => {}
        }
        false
    }

    fn update(&mut self,delta: Duration, castle: &Castle) {
        use Key::*;

        let speed = 250.0 * delta.as_secs_f32();
        if castle.key(W) {self.position.y -= speed}
        if castle.key(A) {self.position.x -= speed}
        if castle.key(S) {self.position.y += speed}
        if castle.key(D) {self.position.x += speed}

        for mut e in &mut self.entities {
            if self.bounce {
                e.pos.x += e.velocity.x * delta.as_secs_f32();
                e.pos.y += e.velocity.y * delta.as_secs_f32();
            }
            if self.rotate {e.angle += e.a_vel * delta.as_secs_f32();}
            if e.pos.x <= 0.0 {e.velocity.x *= -1.0}
            if e.pos.y <= 0.0 {e.velocity.y *= -1.0}
            if e.pos.x >= self.config.width as f32 - e.size.x {e.velocity.x *= -1.0}
            if e.pos.y >= self.config.height as f32 - e.size.y {e.velocity.y *= -1.0}
        }
    }

    fn render(&mut self, g: &mut Graphics) {
        if self.clear_cache {
            self.clear_cache = false;
            g.clear_texture_cache();
        }
        if self.clear {g.clear();} // clears shape vector ;; shape is a struct with x,y,w,h,r,g,b,angle,kind of shape
        g.set_translation(self.position.x,self.position.y);
        g.image("blob.png",0.0,0.0);
        g.image("happy-tree.png",500.0,500.0);
        g.image("token.png", 200.0,0.0);
        g.image("happy-tree-alpha.png",500.0,200.0);
        let r = &mut self.rng;
        for e in &mut self.entities {
            if self.flash {e.color = Color::rgb(r.gen(),r.gen(),r.gen());}
            g.color(e.color);
            g.set_rotation(e.angle);
            g.shape(                // pushes a new shape to the vector, with some calculation from state of Graphics.
            e.kind,
            e.pos.x,
            e.pos.y,
            e.size.x,
            e.size.y
            );
        }
        g.color(Color::WHITE);
        g.rotate(self.timer.elapsed().as_secs_f32());
        g.image("blob.png",110.0,220.0);
        g.set_rotation(0.0);
    }
}

fn main() {
    granseal_engine::start(GameState::new());
}