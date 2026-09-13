use bytemuck::{Pod, Zeroable};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::backend::{BackendType, ComputeBackend, PipelineId};

/// GPU representation of a board bounding box
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable, Serialize, Deserialize)]
pub struct GpuBBox {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub layer: u32,
    pub net_id: u32,
    pub object_type: u32, // 0=track, 1=via, 2=pad, 3=zone
    pub object_id: u32,
}

/// GPU representation of a clearance violation
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable, Serialize, Deserialize)]
pub struct GpuViolation {
    pub obj_a: u32,
    pub obj_b: u32,
    pub distance: f32,
    pub required: f32,
    pub rule_type: u32,
    pub x: f32,
    pub y: f32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct DrcParams {
    pub num_objects: u32,
    pub min_clearance: f32,
    pub _pad0: u32,
    pub _pad1: u32,
}

pub struct GpuDrcChecker {
    backend: Box<dyn ComputeBackend>,
    pipeline_id: PipelineId,
}

impl GpuDrcChecker {
    pub fn new(mut backend: Box<dyn ComputeBackend>) -> Self {
        let pipeline_id = PipelineId(101);
        let shader = include_str!("shaders/drc.wgsl");
        let _ = backend.create_compute_pipeline(pipeline_id, shader, "main");

        Self {
            backend,
            pipeline_id,
        }
    }

    pub fn backend(&self) -> &dyn ComputeBackend {
        &*self.backend
    }

    pub fn check_clearance(
        &mut self,
        objects: &[GpuBBox],
        min_clearance: f32,
    ) -> Result<Vec<GpuViolation>, crate::backend::ComputeError> {
        if objects.is_empty() {
            return Ok(Vec::new());
        }

        if self.backend.backend_type() == BackendType::Cpu {
            return Ok(Self::check_clearance_cpu(objects, min_clearance));
        }

        let num_objects = objects.len() as u32;
        let objects_buffer = self.backend.allocate_buffer(objects)?;

        let max_violations = (objects.len() * 16).clamp(1024, 131072);
        let violations_init = vec![GpuViolation::default(); max_violations];
        let violations_buffer = self.backend.allocate_buffer(&violations_init)?;
        let count_buffer = self.backend.allocate_buffer(&[0u32])?;

        let params = DrcParams {
            num_objects,
            min_clearance,
            _pad0: 0,
            _pad1: 0,
        };
        let params_buffer = self.backend.allocate_buffer(&[params])?;

        let workgroups_x = (num_objects + 15) / 16;
        let workgroups_y = (num_objects + 15) / 16;

        self.backend.dispatch(
            self.pipeline_id,
            [workgroups_x, workgroups_y, 1],
            &[objects_buffer, violations_buffer, count_buffer, params_buffer],
        )?;

        self.backend.synchronize()?;

        let count: Vec<u32> = self.backend.download(count_buffer, 1)?;
        let total_violations = (count[0] as usize).min(max_violations);

        let all_violations: Vec<GpuViolation> = self.backend.download(violations_buffer, total_violations)?;
        Ok(all_violations)
    }

    pub fn check_clearance_cpu(objects: &[GpuBBox], min_clearance: f32) -> Vec<GpuViolation> {
        let n = objects.len();
        (0..n)
            .into_par_iter()
            .flat_map(|i| {
                let mut local = Vec::new();
                let obj_a = &objects[i];
                for obj_b in objects.iter().skip(i + 1) {
                    if obj_a.net_id == obj_b.net_id && obj_a.net_id != 0 {
                        continue;
                    }
                    if obj_a.layer != obj_b.layer {
                        continue;
                    }

                    let expanded_min_x = obj_a.min_x - min_clearance;
                    let expanded_min_y = obj_a.min_y - min_clearance;
                    let expanded_max_x = obj_a.max_x + min_clearance;
                    let expanded_max_y = obj_a.max_y + min_clearance;

                    let overlap = expanded_min_x <= obj_b.max_x
                        && expanded_max_x >= obj_b.min_x
                        && expanded_min_y <= obj_b.max_y
                        && expanded_max_y >= obj_b.min_y;

                    if !overlap {
                        continue;
                    }

                    let dx = (obj_a.min_x - obj_b.max_x).max(obj_b.min_x - obj_a.max_x).max(0.0);
                    let dy = (obj_a.min_y - obj_b.max_y).max(obj_b.min_y - obj_a.max_y).max(0.0);
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist < min_clearance {
                        local.push(GpuViolation {
                            obj_a: obj_a.object_id,
                            obj_b: obj_b.object_id,
                            distance: dist,
                            required: min_clearance,
                            rule_type: 0,
                            x: (obj_a.min_x + obj_a.max_x + obj_b.min_x + obj_b.max_x) * 0.25,
                            y: (obj_a.min_y + obj_a.max_y + obj_b.min_y + obj_b.max_y) * 0.25,
                            _pad: 0,
                        });
                    }
                }
                local
            })
            .collect()
    }
}
