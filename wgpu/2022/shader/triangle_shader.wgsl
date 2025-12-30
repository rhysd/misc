struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

// 0 -> (0.5, -0.5), 1 -> (0.0, 0.5), 2 -> (-0.5, -0.5)
//
//          y
//          ^
//          |
//          . 1 (0, 0.5)
//          |
//          |
// --------------------->x
//          |
//          |
//   .      |      . 0 (0.5, -0.5)
//   2      |
// (-0.5, -0.5)
//          |

[[stage(vertex)]]
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
