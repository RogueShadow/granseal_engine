use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use wgpu::{Device, Queue};
use crate::{Texture, TextureInfo};

#[derive(Copy,Clone,Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub const fn  new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {r,g,b,a}
    }
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r,g,b,1.0)
    }
    pub const BLACK: Self = Self::rgb(0.0,0.0,0.0);
    pub const WHITE: Self = Self::rgb(1.0,1.0,1.0);
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const LIME: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0,0.0,1.0);
    pub const YELLOW: Self = Self::rgb(1.0,1.0,0.0);
    pub const CYAN: Self = Self::rgb(0.0,1.0,1.0);
    pub const MAGENTA: Self = Self::rgb(1.0,0.0,1.0);
    pub const SILVER: Self =  Self::rgb(0.75,0.75,0.75);
    pub const GRAY: Self = Self::rgb(0.5,0.5,0.5);
    pub const MAROON: Self = Self::rgb(0.5,0.0,0.0);
    pub const OLIVE: Self = Self::rgb(0.5,0.5,0.0);
    pub const GREEN: Self = Self::rgb(0.0,0.5,0.0);
    pub const PURPLE: Self = Self::rgb(0.5,0.0,0.5);
    pub const TEAL: Self = Self::rgb(0.0,0.5,0.5);
    pub const NAVY: Self = Self::rgb(0.0,0.0,0.5);
}

pub type ShapeKind = i32;
pub const FILL_RECT: ShapeKind = 0;
pub const FILL_OVAL: ShapeKind = 1;
pub const RECT:  ShapeKind = 2;
pub const OVAL: ShapeKind = 3;
pub const TEX_RECT: ShapeKind = 4;
pub const TEX_OVAL: ShapeKind = 5;

#[repr(C)]
#[derive(Copy,Clone,Debug,bytemuck::Pod,bytemuck::Zeroable)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}
impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ]
        }
    }
}
#[repr(C)]
#[derive(Copy,Clone,Debug,bytemuck::Pod,bytemuck::Zeroable)]
pub struct Quad {
    pub tl: Vertex,
    pub tr: Vertex,
    pub br: Vertex,
    pub bl: Vertex,
}


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
    pub angle: f32,
    pub kind: ShapeKind,
}

impl Shape {
    pub fn new(x: f32, y: f32, width: f32, height: f32, red: f32, green: f32, blue: f32, alpha: f32, angle: f32, kind: ShapeKind) -> Self {
        Self { x, y, width, height, red, green, blue, alpha, angle, kind }
    }
    pub fn fill_rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self::new(x, y, w, h, 1.0, 1.0, 1.0, 1.0, 0.0,FILL_RECT)
    }
    pub fn fill_oval(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self::new(x, y, w, h, 1.0, 1.0, 1.0, 1.0,0.0,  FILL_OVAL)
    }
    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self::new(x, y, w, h, 1.0, 1.0, 1.0, 1.0, 0.0, RECT)
    }
    pub fn oval(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self::new(x, y, w, h, 1.0, 1.0, 1.0, 1.0, 0.0,OVAL)
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
    pub fn angle(mut self, a: f32) -> Self {
        self.angle = a;
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
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Sint32,
                },
            ]
        }
    }
}

//TODO builder, for purposes of having state in the building process.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Graphics {
    pub(crate) device: Rc<wgpu::Device>,
    pub(crate) queue: Rc<wgpu::Queue>,
    pub fill_color: Color,
    pub outline_color: Color,
    pub outline: bool,
    pub outline_thickness: f32,
    pub shapes: Vec<Shape>,
    pub position: [f32; 4],
    // x, y, angle, layer(someday)
    positions: Vec<[f32; 4]>,
    pub(crate) images: HashMap<usize,String>,
    pub(crate) textures: HashMap<String, crate::TextureInfo>,
    pub(crate) texture_bind_group_layout: wgpu::BindGroupLayout,
}


#[allow(dead_code)]
impl Graphics {
    pub fn new(device: Rc<Device>,queue: Rc<Queue>) -> Self {
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
        Self {
            device,
            queue,
            fill_color: Color::WHITE,
            outline_color: Color::BLACK,
            outline: false,
            outline_thickness: 1.0,
            shapes: vec![],
            position: [0.0,0.0,0.0,0.0],
            positions: vec![],
            images: HashMap::new(),
            textures: HashMap::new(),
            texture_bind_group_layout,

        }
    }
    fn info<P>(&mut self,image: P) -> Option<&TextureInfo> where P: AsRef<Path> {
        let path = image.as_ref().clone().to_str().unwrap();
        return self.textures.get(path)
    }
    pub fn load<P>(&mut self, image: P) where P: AsRef<Path> {
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
    pub fn clear(&mut self) -> &Self {
        self.shapes.clear();
        self.images.clear();
        self.positions.clear();
        self
    }
    pub fn color(&mut self, color: Color) -> &Self {
        self.fill_color = color;
        self
    }
    pub fn outline_color(&mut self, color: Color) -> &Self {
        self.outline_color = color;
        self
    }
    pub fn outline_thickness(&mut self, thickness: f32) -> &Self {
        self.outline_thickness = thickness;
        self
    }
    pub fn outline(&mut self, value: bool) -> &Self {
        self.outline = value;
        self
    }
    pub fn set_rotation(&mut self, angle: f32) -> &Self {
        self.position[2] = angle;
        self
    }
    pub fn rotate(&mut self, amount: f32) -> &Self {
        self.position[2] += amount;
        self
    }
    pub fn set_translation(&mut self, x: f32, y: f32) -> &Self {
        self.position[0] = x;
        self.position[1] = y;
        self
    }
    pub fn translate(&mut self, x: f32,  y: f32) -> &Self {
        self.position[0] += x;
        self.position[1] += y;
        self
    }
    pub fn push_position(&mut self) -> &Self {
        self.positions.push(self.position);
        self
    }
    pub fn pop_position(&mut self) -> &Self {
        if !self.positions.is_empty() {
            self.position = self.positions.pop().unwrap();
        }
        self
    }
    fn apply_position(&self, x: f32, y: f32, a: f32) -> (f32,f32,f32) {
        return (self.position[0] + x,self.position[1] + y, self.position[2] + a);
    }

    pub fn shape(&mut self, k: ShapeKind, x: f32, y: f32, width: f32, height: f32) -> &Self {
        let (x,y,a) = self.apply_position(x,y,0.0);
        self.shapes.push(
            Shape::rect(x,y,width,height)
                .color(self.fill_color)
                .angle(a)
                .kind(k)
        );
        self
    }
    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) -> &Self {
        let (x,y,a) = self.apply_position(x,y,0.0);
        self.shapes.push(
          Shape::rect(x,y,width,height)
              .color(self.fill_color)
              .angle(a)
        );
        self
    }
    pub fn oval(&mut self, x: f32, y: f32, width: f32, height: f32) -> &Self {
        let (x,y,a) = self.apply_position(x,y,0.0);
        self.shapes.push(
            Shape::oval(x,y,width,height)
                .color(self.fill_color)
                .angle(a)
        );
        self
    }
    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) -> &Self {
        let (x,y,a) = self.apply_position(x,y,0.0);

        self.shapes.push(
            Shape::fill_rect(x, y, width, height)
            .color(self.fill_color)
            .angle(a)
        );

        if self.outline {
            self.shapes.push(
                Shape::rect(x,y,width,height)
                    .color(self.outline_color)
                    .angle(a)
            );
        }
        self
    }
    pub fn fill_oval(&mut self, x: f32, y: f32, width: f32, height: f32) -> &Self {
        let (x,y,a) = self.apply_position(x,y,0.0);

        self.shapes.push(
            Shape::fill_oval(x, y, width, height)
                .color(self.fill_color)
                .angle(a)
        );

        if self.outline {
            self.shapes.push(
                Shape::oval(x,y,width,height)
                    .color(self.outline_color)
                    .angle(a)
            );
        }
        self
    }
    pub fn image(&mut self,img: &str, x: f32, y: f32) -> &Self {
        let (x,y,a) = self.apply_position(x,y,0.0);

        let tex_info = self.textures.get(img).unwrap();

        self.shapes.push(
            Shape::rect(x,y,tex_info.width as f32,tex_info.height as f32)
                .kind(TEX_RECT)
                .color(self.fill_color)
                .angle(a)
        );
        self.images.insert(self.shapes.len()-1,String::from(img));


        self
    }
}