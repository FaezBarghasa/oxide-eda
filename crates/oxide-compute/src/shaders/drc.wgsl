// oxide-compute/src/shaders/drc.wgsl

// Bounding box structure for spatial queries
struct BBox {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
    layer: u32,
    net_id: u32,
    object_type: u32,  // 0=track, 1=via, 2=pad, 3=zone
    object_id: u32,
}

struct Violation {
    obj_a: u32,
    obj_b: u32,
    distance: f32,
    required: f32,
    rule_type: u32,
    x: f32,
    y: f32,
    _pad: u32,
}

struct Params {
    num_objects: u32,
    min_clearance: f32,
    _pad0: u32,
    _pad1: u32,
}

@group(0) @binding(0) var<storage, read> objects: array<BBox>;
@group(0) @binding(1) var<storage, read_write> violations: array<Violation>;
@group(0) @binding(2) var<storage, read_write> violation_count: atomic<u32>;
@group(0) @binding(3) var<uniform> params: Params;

fn boxes_too_close(a: BBox, b: BBox, clearance: f32) -> bool {
    let expanded_min_x = a.min_x - clearance;
    let expanded_min_y = a.min_y - clearance;
    let expanded_max_x = a.max_x + clearance;
    let expanded_max_y = a.max_y + clearance;
    
    return (expanded_min_x <= b.max_x && expanded_max_x >= b.min_x &&
            expanded_min_y <= b.max_y && expanded_max_y >= b.min_y);
}

fn box_distance(a: BBox, b: BBox) -> f32 {
    let dx = max(max(a.min_x - b.max_x, b.min_x - a.max_x), 0.0);
    let dy = max(max(a.min_y - b.max_y, b.min_y - a.max_y), 0.0);
    return sqrt(dx * dx + dy * dy);
}

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let i = global_id.x;
    let j = global_id.y;
    
    if (i >= params.num_objects || j >= params.num_objects) {
        return;
    }
    
    // Only check upper triangle (i < j)
    if (i >= j) {
        return;
    }
    
    let obj_a = objects[i];
    let obj_b = objects[j];
    
    // Skip same net
    if (obj_a.net_id == obj_b.net_id && obj_a.net_id != 0u) {
        return;
    }
    
    // Skip different layers (for 2D clearance check)
    if (obj_a.layer != obj_b.layer) {
        return;
    }
    
    if (!boxes_too_close(obj_a, obj_b, params.min_clearance)) {
        return;
    }
    
    let dist = box_distance(obj_a, obj_b);
    
    if (dist < params.min_clearance) {
        let idx = atomicAdd(&violation_count, 1u);
        if (idx < arrayLength(&violations)) {
            violations[idx].obj_a = obj_a.object_id;
            violations[idx].obj_b = obj_b.object_id;
            violations[idx].distance = dist;
            violations[idx].required = params.min_clearance;
            violations[idx].rule_type = 0u; // clearance
            violations[idx].x = (obj_a.min_x + obj_a.max_x + obj_b.min_x + obj_b.max_x) * 0.25;
            violations[idx].y = (obj_a.min_y + obj_a.max_y + obj_b.min_y + obj_b.max_y) * 0.25;
            violations[idx]._pad = 0u;
        }
    }
}
