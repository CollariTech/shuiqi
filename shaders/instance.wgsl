struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
}

struct InstanceInput {
    @location(2) position: vec2<f32>,
    @location(3) scale: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput, instance: InstanceInput) -> VertexOutput {
    var output: VertexOutput;
    let scaled_position = input.position * instance.scale;
    let world_position = scaled_position + instance.position;

    output.clip_position = vec4<f32>(world_position, 0.0, 1.0);
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(out: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(out.color, 0.0);
}