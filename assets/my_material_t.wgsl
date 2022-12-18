//https://github.com/bevyengine/bevy/blob/c2da7800e3671ad92e775529070a814d0bc2f5f8/crates/bevy_sprite/src/mesh2d/mesh2d.wgsl
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

// let MAX_FIRES = 10;

struct MyMat {
    color: vec4<f32>,
    time: f32,
    // position: array<vec4<f32>, MAX_FIRES>,
}

@group(1) @binding(0)
var<uniform> uniform_data: MyMat;

fn circle(st: vec2<f32>, center: vec2<f32>, radius: f32) -> f32{
    let dist = st-center;
    let smoothness = 0.0;
	return smoothstep(radius-(radius*smoothness),
                         radius+(radius*smoothness),
                         dot(dist,dist)*4.0);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    // var output_color = vec4<f32>(1.0, 1.0, 1.0, uniform_data.time);
    // output_color = output_color * textureSample(texture, our_sampler, input.uv);
    // output_color = output_color * uniform_data.color;
    var output_color = vec4<f32>(0.0,0.0,0.0, 1.0);
    // output_color = output_color * uniform_data.color;
    // for( var i: i32 = 0; i < MAX_FIRES; i= i +1) {
    //     if (uniform_data.position[i].z == 0.0) {
    //         continue;
    //     }
        
    //     output_color = output_color * ( circle (input.world_position.xy, uniform_data.position[i].xy, uniform_data.position[i].z) );
    // }
    return output_color;
}


