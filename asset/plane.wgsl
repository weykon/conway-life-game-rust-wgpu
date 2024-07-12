struct BackgroudBuffer {
    width: u32,
    height: u32,
    block_w: u32,
    block_h: u32,
}

@group(0) @binding(0) var<uniform> uniforms : BackgroudBuffer;

alias TriangleVertices = array<vec2f, 6>;
var<private> vertices : TriangleVertices = TriangleVertices(
    vec2f(-1.0, 1.0),
    vec2f(-1.0, -1.0),
    vec2f(1.0, 1.0),
    vec2f(1.0, 1.0),
    vec2f(-1.0, -1.0),
    vec2f(1.0, -1.0),
);

@vertex 
fn display_vs(@builtin(vertex_index) vid: u32) -> @builtin(position) vec4f {
    return vec4f(vertices[vid], 0.0, 1.0);
}

@fragment
fn display_fs(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    let aspect_ratio = f32(uniforms.width) / f32(uniforms.height);
    let grid_line_width_ratio: f32 = 0.02;

    var uv = pos.xy / vec2f(f32(uniforms.width - 1u), f32(uniforms.height - 1u));

    // block size ratio
    let from_source_xy_ratio = f32(uniforms.block_w / uniforms.block_h);
    let b_w = f32(uniforms.block_w) / f32(uniforms.width);
    let b_h = b_w / from_source_xy_ratio;

    var color = vec3(0.0);

    uv = modf(uv / vec2(b_w,b_h)).fract;

    let left_bottom = step(vec2(grid_line_width_ratio-1.), uv);
    var sum = left_bottom.x * left_bottom.y;

    let right_top = step(uv, vec2(1.0 - grid_line_width_ratio));
    sum *= right_top.x * right_top.y;

    color = vec3(sum);

    return vec4f(color, 1.0);
}