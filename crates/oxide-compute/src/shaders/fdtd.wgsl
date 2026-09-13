// oxide-compute/src/shaders/fdtd.wgsl

struct FieldCell {
    ex: f32,
    ey: f32,
    ez: f32,
    hx: f32,
    hy: f32,
    hz: f32,
    _pad0: f32,
    _pad1: f32,
}

struct FdtdParams {
    grid_x: u32,
    grid_y: u32,
    grid_z: u32,
    dt: f32,
    dx: f32,
    epsilon: f32,
    mu: f32,
    _pad: u32,
}

@group(0) @binding(0) var<storage, read_write> fields: array<FieldCell>;
@group(0) @binding(1) var<uniform> params: FdtdParams;

fn cell_index(x: u32, y: u32, z: u32) -> u32 {
    return z * params.grid_x * params.grid_y + y * params.grid_x + x;
}

@compute @workgroup_size(4, 4, 4)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;
    let z = global_id.z;
    
    if (x >= params.grid_x || y >= params.grid_y || z >= params.grid_z) {
        return;
    }
    
    let idx = cell_index(x, y, z);
    var cell = fields[idx];
    
    // Maxwell Curl Equations update: dE/dt = (1/eps) * curl(H)
    if (x > 0u && y > 0u && z > 0u) {
        let idx_xm = cell_index(x - 1u, y, z);
        let idx_ym = cell_index(x, y - 1u, z);
        let idx_zm = cell_index(x, y, z - 1u);
        
        let cell_xm = fields[idx_xm];
        let cell_ym = fields[idx_ym];
        let cell_zm = fields[idx_zm];
        
        let inv_dx = 1.0 / params.dx;
        let inv_eps = 1.0 / params.epsilon;
        
        let dhz_dy = (cell.hz - cell_ym.hz) * inv_dx;
        let dhy_dz = (cell.hy - cell_zm.hy) * inv_dx;
        let dhx_dz = (cell.hx - cell_zm.hx) * inv_dx;
        let dhz_dx = (cell.hz - cell_xm.hz) * inv_dx;
        let dhy_dx = (cell.hy - cell_xm.hy) * inv_dx;
        let dhx_dy = (cell.hx - cell_ym.hx) * inv_dx;
        
        cell.ex += params.dt * inv_eps * (dhz_dy - dhy_dz);
        cell.ey += params.dt * inv_eps * (dhx_dz - dhz_dx);
        cell.ez += params.dt * inv_eps * (dhy_dx - dhx_dy);
    }
    
    fields[idx] = cell;
}
