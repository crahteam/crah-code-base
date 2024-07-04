struct Camera {
    view_proj: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>, // NEW!
}
struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
    @location(3) position: vec3<f32>
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.world_normal = model.normal;
    var world_position: vec4<f32> = model_matrix * vec4<f32>(model.position, 1.0);
    out.world_position = world_position.xyz;
    out.clip_position = camera.view_proj * world_position;
    out.position = model.position;
    return out;
}

// Fragment shader

@group(0)@binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //before adding light
    //return textureSample(t_diffuse, s_diffuse, in.tex_coords);
	let object_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    
    
    // FOG
    
	let d = distance(in.world_position, in.position);
	let fog_factor = get_fog(d);
	
    //
    
    // We don't need (or want) much ambient light, so 0.1 is fine
    let ambient_strength = 0.3;
   
    let ambient_color = light.color * ambient_strength;
	let light_dir = normalize(light.position - in.world_position);

	let diffuse_strength = max(dot(in.world_normal, light_dir), 0.0);
	
	let diffuse_color = light.color * diffuse_strength;
    let result = (ambient_color + diffuse_color) * object_color.xyz;
	//mix(result, white * fog_factor, 0.0)
	let white = vec3<f32>(1.0, 1.0, 1.0);
    return vec4<f32>(mix(result, white * fog_factor, 0.7), object_color.a);
}

fn get_fog(d: f32) -> f32 {

	let fmin: f32 = 0.0;
	let fmax: f32 = 10.0;
	
	// edge cases
	
	if d >= fmax {
		return 1.0;
	}
	//if d < fmin {
	//	return 0.0;
	//}


    return (1.0 - (fmax - d) / (fmax - fmin));
}

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}

@group(2) @binding(0)
var<uniform> light: Light;
