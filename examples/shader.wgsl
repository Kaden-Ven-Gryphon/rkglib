// Vertex shader

struct VertextOutput {
	@builtin(position) clip_posiiont: vec4<f32>,
	@location(0) vert_pos: vec3<f32>,
}

@vertex
fn vs_main(
	@builtin(vertex_index) in_vertex_index: u32,
) -> VertextOutput {
	var out: VertextOutput;
	let x = f32(1 - i32(in_vertex_index)) * 0.5;
	let y = f32(i32(in_vertex_index & 1u) * 2 -1) *0.5;
	out.clip_posiiont = vec4<f32>(x,y,0.0,1.0);
	out.vert_pos = out.clip_posiiont.xyz;
	return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertextOutput) -> @location(0) vec4<f32> {
	return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}