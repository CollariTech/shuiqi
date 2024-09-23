struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) rgb_color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    out.color = model.rgb_color;
    return out;
}

@fragment
fn fs_main(out: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(out.color, 0.0);
}