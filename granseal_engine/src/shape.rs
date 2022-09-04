use wgpu::util::DeviceExt;

#[derive(Copy,Clone,Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub const fn  new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r,g,b,a,
        }
    }
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
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


pub type ShapeKind = i32;
pub const FILL_RECT: ShapeKind = 0;
pub const FILL_OVAL: ShapeKind = 1;
pub const RECT:  ShapeKind = 2;
pub const OVAL: ShapeKind = 3;
pub const TEX_RECT: ShapeKind = 4;//TODO implement textured rect.
pub const TEX_OVAL: ShapeKind = 5;//TODO implement textured oval.

#[repr(C)]
#[derive(Copy,Clone,Debug,bytemuck::Pod,bytemuck::Zeroable)]
pub struct Shape {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
    pub kind: ShapeKind,
}

impl Shape {
    pub fn new(x: f32, y: f32, w: f32, h: f32,r: f32, g: f32, b: f32,a: f32, k: ShapeKind) -> Self {
        Self { x, y, width: w, height: h, red: r, green: g, blue: b, alpha: a, kind: k }
    }
    pub fn fill_square(x: f32, y: f32, s: f32) -> Self {
        Self::new(x, y, s, s, 1.0, 1.0, 1.0, 1.0, FILL_RECT)
    }
    pub fn fill_rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self::new(x, y, w, h, 1.0, 1.0, 1.0, 1.0, FILL_RECT)
    }
    pub fn fill_circle(x: f32, y: f32, r: f32) -> Self {
        Self::new(x, y, r, r, 1.0, 1.0, 1.0, 1.0, FILL_OVAL)
    }
    pub fn fill_oval(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self::new(x, y, w, h, 1.0, 1.0, 1.0, 1.0, FILL_OVAL)
    }
    pub fn square(x: f32, y: f32, s: f32) -> Self {
        Self::new(x, y, s, s, 1.0, 1.0, 1.0, 1.0, RECT)
    }
    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self::new(x, y, w, h, 1.0, 1.0, 1.0, 1.0, RECT)
    }
    pub fn circle(x: f32, y: f32, r: f32) -> Self {
        Self::new(x, y, r, r, 1.0, 1.0, 1.0, 1.0, OVAL)
    }
    pub fn oval(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self::new(x, y, w, h, 1.0, 1.0, 1.0, 1.0, OVAL)
    }
    pub fn rgb(mut self, r: f32, g: f32, b: f32) -> Self {
        self.red = r;
        self.green = g;
        self.blue = b;
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.red = color.r;
        self.green = color.g;
        self.blue = color.b;
        self
    }
    pub fn opacity(mut self, a: f32) -> Self {
        self.alpha = a;
        self
    }
    pub fn kind(mut self, k: ShapeKind) -> Self {
        self.kind = k;
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

//TODO builder, for purposes of having state in the building process.
pub struct ShapeBuilder {
    pub fill_color: Color,
    pub outline_color: Color,
    pub outline_thickness: f32,
    pub kind: ShapeKind,
}

impl ShapeBuilder {
    fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }
    fn outline_color(mut self, color: Color) -> Self {
        self.outline_color = color;
        self
    }
    fn outline_thickness(mut self, thickness: f32) -> Self {
        self.outline_thickness = thickness;
        self
    }
    fn kind(mut self, kind: ShapeKind) -> Self {
        self.kind = kind;
        self
    }
}