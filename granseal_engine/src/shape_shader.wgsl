struct ScreenUniform {
    width: f32,
    height: f32,
}

struct VertexInput {
    @location(0) rect: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) angle: f32,
    @location(3) kind: i32,
}

fn check_oval(h: f32, k: f32, x: f32, y: f32, a:  f32, b: f32) -> bool {
    return ((pow(x-h,2.0) / pow(a,2.0)) + (pow(y-k,2.0) / pow(b,2.0))) >= 1.0;
}
fn check_oval2(h: f32, k: f32, x: f32, y: f32, a:  f32, b: f32) -> f32 {
    return ((pow(x-h,2.0) / pow(a,2.0)) + (pow(y-k,2.0) / pow(b,2.0)));
}

@group(0) @binding(0)
var<uniform> screen: ScreenUniform;
@group(0) @binding(1)
var<uniform> timer: f32;

@group(1) @binding(0)
var t: texture_2d<f32>;
@group(1) @binding(1)
var s: sampler;


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) @interpolate(perspective,center) color: vec4<f32>,
    @location(1) rect: vec4<f32>,
    @location(2) kind: i32,
    @location(3) @interpolate(linear,center) tex_coords: vec2<f32>,
};

fn convert(x: f32, y: f32, screen: ScreenUniform) -> vec2<f32> {
    let nx = ((x / screen.width) * 2.0) - 1.0;
    let ny = ((y / screen.height) * 2.0) - 1.0;
    return vec2<f32>(nx,ny);
}

@vertex
fn vs_main(@builtin(vertex_index) index: u32, in: VertexInput) -> VertexOutput {
    let rotation = mat2x2<f32>(cos(in.angle),-sin(in.angle),sin(in.angle),cos(in.angle));

    //x y in screen coordinates 0,0 is top left.
    var screen_position =  vec2<f32>(in.rect[0],in.rect[1]);
    //width height in screen coordinates.
    var screen_size = vec2<f32>(in.rect[2],in.rect[3]);

    var p1 = convert(screen_position.x,screen_position.y,screen);
    var pcenter = convert(screen_position.x - screen_size.x/2.0, screen_position.y + screen_size.y/2.0, screen);
    var p2 = convert(screen_position.x + screen_size.x,screen_position.y + screen_size.y, screen);

    var x = 0.0;
    var y = 0.0;
    var out: VertexOutput;

    switch (index) {  // construct a triangle strip of two triangles from the index using 2 points from above.
        case 0u, 4u: {      // bottom left
            x = p1.x;
            y = p1.y;
            out.tex_coords = vec2(0.0,0.0);
        }
        case 1u: {       //  top left
            x = p1.x;
            y = p2.y;
            out.tex_coords = vec2(0.0,1.0);
        }
        case 2u: {      // top right
            x = p2.x;
            y = p2.y;
            out.tex_coords = vec2(1.0,1.0);
        }
        case 3u: {    // bottom right
            x = p2.x;
            y = p1.y;
            out.tex_coords = vec2(1.0,0.0);
        }
        default: {}
    }
    y = -y;

    var clip_pos = rotation * vec2<f32>(x,y);

    out.clip_position = vec4<f32>(clip_pos,0.0,1.0);
    out.rect = in.rect;
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
            in.rect[0] + in.rect[2]/2.0,
            in.rect[1] + in.rect[3]/2.0,
            in.clip_position[0],
            in.clip_position[1],
            in.rect[2] / 2.0,
            in.rect[3] / 2.0,
        );
        if (check) {
            color_out.a = 0.0;
        }
    }
    if (in.kind == 2) {
        let thickness = 4.0;

        let centerx = in.rect[0] + in.rect[2]/2.0;
        let centery = in.rect[1] + in.rect[3]/2.0;

        let dx = abs(in.clip_position[0] - centerx) + thickness;
        let dy = abs(in.clip_position[1] - centery) + thickness;

        if (dx > in.rect[2]/2.0  || dy > in.rect[3]/2.0 ) {} else {
            color_out.a = 0.0;
        }

    }
    if (in.kind == 3) {
        let thickness = 4.0 / 10.0;

        let check = check_oval2(
            in.rect[0] + in.rect[2]/2.0,
            in.rect[1] + in.rect[3]/2.0,
            in.clip_position[0],
            in.clip_position[1],
            in.rect[2] / 2.0,
            in.rect[3] / 2.0,
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
            in.rect[0] + in.rect[2]/2.0,
            in.rect[1] + in.rect[3]/2.0,
            in.clip_position[0],
            in.clip_position[1],
            in.rect[2] / 2.0,
            in.rect[3] / 2.0,
        );
        color_out = diffuse_color * color_out;
        if (check) {
            color_out.a = 0.0;
        }
    }

    return color_out;
}