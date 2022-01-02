struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] pos: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;

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

    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);

    let r = f32(in_vertex_index & 1u);
    let g = f32(in_vertex_index & 2u);
    let b = f32(!in_vertex_index);
    out.pos = vec3<f32>(r, g, b);

    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.pos, 1.0);
}
