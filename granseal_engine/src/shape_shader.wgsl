fn border(st: vec2<f32>) -> f32 {
    var tl = step(vec2<f32>(0.05,0.05),st);
    var br = step(vec2<f32>(0.05,0.05),1.0 - st);
    return tl.x * tl.y * br.x * br.y;
}

struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) angle: f32,
    @location(4) kind: i32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) @interpolate(linear,center) color: vec4<f32>,
    @location(1) pos: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) kind: i32,
    @location(4) @interpolate(linear,center) tex_coords: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> screen: vec2<f32>;
@group(0) @binding(1)
var<uniform> timer: f32;
@group(0) @binding(2)
var<uniform> ortho: mat4x4<f32>;

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
    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // n, normalized to 0.0-1.0
    // rn, relative-normalized to 0.0-1.0 from 0.0 at shapes position to 1.0 at shapes size
    //    var PixelPos = in.clip_position.xy;
    //    var ShapePos = in.pos.xy;
    //    var ShapeSize = in.size.xy;
    //    var nPixelPos = PixelPos / screen;
    //    var nShapePos = ShapePos / screen ;
    //    var nShapeSize = ShapeSize / screen ;
    //    var rnShapePos = ((PixelPos - (ShapePos - ShapeSize/4.0)) / ShapeSize);
    //    var ndcShapePos = (rnShapePos *  2.0 - 1.0);


    var diffuse_color = textureSample(t,s,in.tex_coords);
    var ndcPos = in.tex_coords * 2.0 - 0.5;
    var color_out = in.color;


    if (in.kind == 0) { // filled rect
        return in.color;
    }
    if (in.kind == 1) { // filled oval
        var pct = distance(vec2<f32>(0.5,0.5),ndcPos);
        pct = step(0.1,1.0 - pct);
        //let alpha = smoothstep(0.0,0.1,pct);
        return in.color * pct;
    }
    if (in.kind == 2) { // rect outline
        var pct = 1.0 - border(in.tex_coords);
        return in.color * pct;
    }
    if (in.kind == 3) { // oval outline
        var d = distance(vec2<f32>(0.5,0.5),ndcPos);
        pct = step(0.0,1.0 - d);
        pct *= step(0.9,d);
        return in.color * pct;
    }
    if (in.kind == 4) { // textured rect
        return in.color * diffuse_color;
    }
    if (in.kind == 5) { // textured oval
        var pct = distance(vec2<f32>(0.5,0.5),ndcPos);
        pct = step(0.1,1.0 - pct);
        //let alpha = smoothstep(0.0,0.1,pct);
        return diffuse_color * in.color * pct;
    }

    return in.color;
}

