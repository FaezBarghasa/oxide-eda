// oxide-compute/src/shaders/congestion.wgsl

struct CongestionParams {
    grid_width: u32,
    grid_height: u32,
    num_tracks: u32,
    cell_size: f32,
}

struct GpuTrack {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    layer: u32,
    width: f32,
    net_id: u32,
    _pad: u32,
}

@group(0) @binding(0) var<storage, read> tracks: array<GpuTrack>;
@group(0) @binding(1) var<storage, read_write> congestion_map: array<atomic<u32>>;
@group(0) @binding(2) var<uniform> params: CongestionParams;

fn world_to_grid(x: f32, y: f32) -> vec2<u32> {
    let gx = max(0.0, x / params.cell_size);
    let gy = max(0.0, y / params.cell_size);
    return vec2<u32>(u32(gx), u32(gy));
}

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let track_idx = global_id.x;
    
    if (track_idx >= params.num_tracks) {
        return;
    }
    
    let track = tracks[track_idx];
    
    let start = world_to_grid(track.x0, track.y0);
    let end = world_to_grid(track.x1, track.y1);
    
    let dx = i32(end.x) - i32(start.x);
    let dy = i32(end.y) - i32(start.y);
    let steps = max(abs(dx), abs(dy));
    
    if (steps == 0) {
        if (start.x < params.grid_width && start.y < params.grid_height) {
            let cell_idx = start.y * params.grid_width + start.x;
            atomicAdd(&congestion_map[cell_idx], 1u);
        }
        return;
    }
    
    for (var i = 0; i <= steps; i++) {
        let t = f32(i) / f32(steps);
        let x = u32(f32(start.x) + t * f32(dx));
        let y = u32(f32(start.y) + t * f32(dy));
        
        if (x < params.grid_width && y < params.grid_height) {
            let cell_idx = y * params.grid_width + x;
            atomicAdd(&congestion_map[cell_idx], 1u);
        }
    }
}
