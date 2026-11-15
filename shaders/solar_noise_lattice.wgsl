// Authored periodic material noise, not an observational dataset. Generated
// once at renderer initialization; no frame-time texture update is required.
@group(0) @binding(2) var solar_noise_output: texture_storage_3d<rgba16float, write>;

@compute @workgroup_size(8, 8, 4)
fn initialize_solar_noise(@builtin(global_invocation_id) id: vec3<u32>) {
    textureStore(solar_noise_output, vec3<i32>(id), vec4<f32>(hash33(vec3<f32>(id)), 1.0));
}
