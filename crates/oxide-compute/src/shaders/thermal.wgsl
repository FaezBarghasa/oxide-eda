// oxide-compute/src/shaders/thermal.wgsl

struct ThermalParams {
    grid_width: u32,
    grid_height: u32,
    ambient_temp: f32,
    dt: f32,
    copper_conductivity: f32,
    fr4_conductivity: f32,
    _pad0: u32,
    _pad1: u32,
}

@group(0) @binding(0) var<storage, read> temp_current: array<f32>;
@group(0) @binding(1) var<storage, read_write> temp_next: array<f32>;
@group(0) @binding(2) var<storage, read> power_map: array<f32>;
@group(0) @binding(3) var<storage, read> material_map: array<u32>; // 0=FR4, 1=Copper
@group(0) @binding(4) var<uniform> params: ThermalParams;

fn get_index(x: u32, y: u32) -> u32 {
    return y * params.grid_width + x;
}

fn get_conductivity(x: u32, y: u32) -> f32 {
    let material = material_map[get_index(x, y)];
    if (material == 1u) {
        return params.copper_conductivity;
    }
    return params.fr4_conductivity;
}

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;
    
    if (x >= params.grid_width || y >= params.grid_height) {
        return;
    }
    
    let idx = get_index(x, y);
    let current = temp_current[idx];
    
    // Boundary conditions: outer perimeter stays fixed at ambient temperature
    if (x == 0u || y == 0u || x == params.grid_width - 1u || y == params.grid_height - 1u) {
        temp_next[idx] = params.ambient_temp;
        return;
    }
    
    let left = temp_current[get_index(x - 1u, y)];
    let right = temp_current[get_index(x + 1u, y)];
    let up = temp_current[get_index(x, y - 1u)];
    let down = temp_current[get_index(x, y + 1u)];
    
    let k_center = get_conductivity(x, y);
    
    // Finite Difference 2D heat equation: dT/dt = alpha * laplacian(T) + Q
    let laplacian = (left + right + up + down - 4.0 * current);
    let heat_source = power_map[idx];
    
    let alpha = k_center;
    let new_temp = current + params.dt * (alpha * laplacian + heat_source);
    
    temp_next[idx] = new_temp;
}
