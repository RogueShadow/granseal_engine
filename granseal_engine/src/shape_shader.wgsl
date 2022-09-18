// Beginnings of a library of shaping functions. Or so.
fn border(st: vec2<f32>, t: vec2<f32>) -> f32 {
    var tl = step(t,st);
    var br = step(t,1.0 - st);
    return tl.x * tl.y * br.x * br.y;
}
fn oval(st: vec2<f32>) -> f32 {
    var pct = distance(vec2<f32>(0.5,0.5),st);
    pct = smoothstep(0.0,0.05,1.0 - pct);
    return pct;
}
struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) angle: f32,
    @location(4) kind: i32,
    @location(5) thickness: f32,
}
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) pos: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) kind: i32,
    @location(4) tex_coords: vec2<f32>,
    @location(5) thickness: f32,
};
@group(0) @binding(0)
var<uniform> screen: vec2<f32>;
@group(0) @binding(1)
var<uniform> timer: f32;
@group(1) @binding(0)
var t: texture_2d<f32>;
@group(1) @binding(1)
var s: sampler;

@vertex
fn vs_main(@builtin(vertex_index) index: u32, in: VertexInput) -> VertexOutput {
    let rotation = mat2x2<f32>(cos(in.angle),-sin(in.angle),sin(in.angle),cos(in.angle));
    let size = vec2<f32>(in.size.x,in.size.y);
    let position = vec2<f32>(in.pos.x,in.pos.y) + size/2.0;
    let translation = (position/screen);
    let translation = (vec2<f32>(translation.x,1.0 - translation.y) - 0.5) * 2.0;
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
    p.y = -p.y;
    out.clip_position = vec4<f32>((rotation * p) / screen + translation,0.0,1.0);
    out.pos = in.pos;
    out.size = in.size;
    out.kind =  in.kind;
    out.color = in.color;
    out.thickness = in.thickness;
    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var thickness = vec2<f32>(in.thickness / in.size.x , in.thickness / in.size.y) / 2.0;
    var diffuse_color = textureSample(t,s,in.tex_coords);
    var ndcPos = in.tex_coords * 2.0 - 0.5; // convert to -1,1 range for some functions
    if (in.kind == 0) {
        return in.color;
    } // filled rect just for completeness.
    if (in.kind == 1) { // filled oval
        return in.color * oval(ndcPos);
    }
    if (in.kind == 2) { // rect outline
        var pct = 1.0 - border(in.tex_coords,thickness);
        return in.color * pct;
    }
    if (in.kind == 3) { // oval outline
        var d = distance(vec2<f32>(0.5,0.5),ndcPos);
        pct = step(0.1 - thickness.x,1.0 - d);
        pct *= step(0.9 - thickness.x,d);
        return in.color * pct;
    }
    if (in.kind == 4) { // textured rect
        return diffuse_color;
    }
    if (in.kind == 5) { // textured oval
        return diffuse_color * in.color * oval(ndcPos);
    }
    return vec4<f32>(1.0,0.5,1.0,1.0);
}

