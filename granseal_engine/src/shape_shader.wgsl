
struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) angle: f32,
    @location(4) kind: i32,
}

fn check_oval(h: f32, k: f32, x: f32, y: f32, a:  f32, b: f32) -> bool {
    return ((pow(x-h,2.0) / pow(a,2.0)) + (pow(y-k,2.0) / pow(b,2.0))) >= 1.0;
}
fn check_oval2(h: f32, k: f32, x: f32, y: f32, a:  f32, b: f32) -> f32 {
    return ((pow(x-h,2.0) / pow(a,2.0)) + (pow(y-k,2.0) / pow(b,2.0)));
}

@group(0) @binding(0)
var<uniform> screen: vec2<f32>;
@group(0) @binding(1)
var<uniform> timer: f32;

@group(1) @binding(0)
var t: texture_2d<f32>;
@group(1) @binding(1)
var s: sampler;


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) @interpolate(linear,center) color: vec4<f32>,
    @location(1) pos: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) kind: i32,
    @location(4) @interpolate(linear,center) tex_coords: vec2<f32>,
};


@vertex
fn vs_main(@builtin(vertex_index) index: u32, in: VertexInput) -> VertexOutput {
    let aspect = screen.x/screen.y;
    let rotation = mat2x2<f32>(cos(in.angle),-sin(in.angle),sin(in.angle),cos(in.angle));
    let size = (((vec2<f32>(in.size.x,in.size.y)  ) / screen) );
    let translation = ((vec2<f32>(in.pos.x,in.pos.y) / screen) - vec2<f32>(0.5,0.5) + size/2.0 ) * 2.0;

    var p = vec2<f32>(0.0,0.0);

    var out: VertexOutput;

    switch (index) {  // construct a triangle strip of two triangles from the index.
        case 0u, 4u: {      // bottom left
            p.x = -size.x;
            p.y = -size.y;
            out.tex_coords = vec2(0.0,0.0);
        }
        case 1u: {       //  top left
            p.x = -size.x;
            p.y = size.y;
            out.tex_coords = vec2(0.0,1.0);
        }
        case 2u: {      // top right
            p.x = size.x;
            p.y = size.y;
            out.tex_coords = vec2(1.0,1.0);
        }
        case 3u: {    // bottom right
            p.x = size.x;
            p.y = -size.y;
            out.tex_coords = vec2(1.0,0.0);
        }
        default: {}
    }

    p = (rotation * p) + translation;

    p.y = -p.y;
    out.clip_position = vec4<f32>(p,0.0,1.0);
    out.pos = in.pos;
    out.size = in.size;
    out.kind =  in.kind;
    out.color = in.color;
    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    var diffuse_color = textureSample(t,s,in.tex_coords);
    var color_out = in.color;

    if (in.kind == 0) {

    }
    if (in.kind == 1) {
        let check = check_oval(
            in.pos.x + in.size.x/2.0,
            in.pos.y + in.size.y/2.0,
            in.clip_position[0],
            in.clip_position[1],
            in.size.x / 2.0,
            in.size.y / 2.0,
        );
        if (check) {
            color_out.a = 0.0;
        }
    }
    if (in.kind == 2) {
        let thickness = 4.0;

        let centerx = in.pos.x + in.pos.x/2.0;
        let centery = in.pos.y + in.pos.y/2.0;

        let dx = abs(in.clip_position[0] - centerx) + thickness;
        let dy = abs(in.clip_position[1] - centery) + thickness;

        if (dx > in.pos.x/2.0  || dy > in.pos.y/2.0 ) {} else {
            color_out.a = 0.0;
        }

    }
    if (in.kind == 3) {
        let thickness = 4.0 / 10.0;

        let check = check_oval2(
            in.pos.x + in.size.x/2.0,
            in.pos.y + in.size.y/2.0,
            in.clip_position[0],
            in.clip_position[1],
            in.size.x / 2.0,
            in.size.y / 2.0,
        );
        if (check >= 1.0 || check <= 1.0 - thickness) {
            color_out.a = 0.0;
        }
    }
    if (in.kind == 4) {
        color_out = diffuse_color * color_out;
    }
    if (in.kind == 5) {
        let check = check_oval(
            in.pos.x + in.size.x/2.0,
            in.pos.y + in.size.y/2.0,
            in.clip_position[0],
            in.clip_position[1],
            in.size.x / 2.0,
            in.size.y / 2.0,
        );
        color_out = diffuse_color * color_out;
        if (check) {
            color_out.a = 0.0;
        }
    }

    return color_out;
}