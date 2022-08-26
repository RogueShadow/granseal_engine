

pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    const fn  new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r,g,b,a,
        }
    }
    const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r,g,b,1.0)
    }
}

pub const BLACK: Color = Color::rgb(0.0,0.0,0.0);
pub const WHITE: Color = Color::rgb(1.0,1.0,1.0);
pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
pub const LIME: Color = Color::rgb(0.0, 1.0, 0.0);
pub const BLUE: Color = Color::rgb(0.0,0.0,1.0);
pub const YELLOW: Color = Color::rgb(1.0,1.0,0.0);
pub const CYAN: Color = Color::rgb(0.0,1.0,1.0);
pub const MAGENTA: Color = Color::rgb(1.0,0.0,1.0);
pub const SILVER: Color =  Color::rgb(0.75,0.75,0.75);
pub const GRAY: Color = Color::rgb(0.5,0.5,0.5);
pub const MAROON: Color = Color::rgb(0.5,0.0,0.0);
pub const OLIVE: Color = Color::rgb(0.5,0.5,0.0);
pub const GREEN: Color = Color::rgb(0.0,0.5,0.0);
pub const PURPLE: Color = Color::rgb(0.5,0.0,0.5);
pub const TEAL: Color = Color::rgb(0.0,0.5,0.5);
pub const NAVY: Color = Color::rgb(0.0,0.0,0.5);

#[repr(C)]
#[derive(Copy,Clone,Debug,bytemuck::Pod,bytemuck::Zeroable)]
pub struct Shape {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub k: i32,
}

impl Shape {
    pub fn new(x: f32, y: f32, w: f32, h: f32,r: f32, g: f32, b: f32,a: f32, k: i32) -> Self {
        Self { x, y, w, h, r, g, b, a, k }
    }
    pub fn square(x: f32, y: f32, s: f32) -> Self {
        Self::new(x,y,s,s,1.0,1.0,1.0,1.0,0)
    }
    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self::new(x,y,w,h,1.0,1.0,1.0,1.0,0)
    }
    pub fn circle(x: f32, y: f32, r: f32) -> Self {
        Self::new(x,y,r,r,1.0,1.0,1.0,1.0,1)
    }
    pub fn oval(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self::new(x,y,w,h,1.0,1.0,1.0,1.0,1)
    }
    pub fn rgb(mut self, r: f32, g: f32, b: f32) -> Self {
        self.r = r;
        self.g = g;
        self.b = b;
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.r = color.r;
        self.g = color.g;
        self.b = color.b;
        self
    }
    pub fn opacity(mut self, a: f32) -> Self {
        self.a = a;
        self
    }
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Shape>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Sint32,
                },
            ]
        }
    }
}