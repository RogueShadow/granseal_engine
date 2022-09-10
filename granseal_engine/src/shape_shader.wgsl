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

fn convert_rect(rect: vec4<f32>, screen: ScreenUniform) -> vec4<f32> {
    let rect = vec4<f32>(
        (rect[0] / screen.width) * 2.0 - 1.0,
        (rect[1] / screen.height) * 2.0 - 1.0,
        (rect[2] / screen.width) * 2.0,
        (rect[3] / screen.height * 2.0)
    );
    return rect;
}

fn check_oval(h: f32, k: f32, x: f32, y: f32, a:  f32, b: f32) -> bool {
    return ((pow(x-h,2.0) / pow(a,2.0)) + (pow(y-k,2.0) / pow(b,2.0))) >= 1.0;
}
fn check_oval2(h: f32, k: f32, x: f32, y: f32, a:  f32, b: f32) -> f32 {
    return ((pow(x-h,2.0) / pow(a,2.0)) + (pow(y-k,2.0) / pow(b,2.0)));
}

@group(0) @binding(0)
var<uniform> screen: ScreenUniform;

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

@vertex
fn vs_main(@builtin(vertex_index) index: u32, in: VertexInput) -> VertexOutput {
    var box = convert_rect(in.rect, screen);
    var out: VertexOutput;
    var x = 0.0;
    var y = 0.0;
    switch (index) {
        case 0u, 4u: {      // bottom left
            x = box[0];
            y = box[1];
            out.tex_coords = vec2(0.0,0.0);
        }
        case 1u: {       //  top left
            x = box[0];
            y = box[1] + box[3];
            out.tex_coords = vec2(0.0,1.0);
        }
        case 2u: {      // top right
            x = box[0] + box[2];
            y = box[1] + box[3];
            out.tex_coords = vec2(1.0,1.0);
        }
        case 3u: {    // bottom right
            x = box[0] + box[2];
            y = box[1];
            out.tex_coords = vec2(1.0,0.0);
        }
        default: {}
    }
    y = -y;
    out.clip_position = vec4<f32>(x,y,0.0,1.0);
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