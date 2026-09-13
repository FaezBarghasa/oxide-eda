//! Spatial Indexing and fast 2D collision acceleration.

use uuid::Uuid;

use oxide_physics::Microns;
use oxide_types::pcb::PcbBoard;

use super::{BoundingBox, Point2D};

pub type ObjectId = Uuid;
pub type NetId = u32;
pub type LayerId = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpatialObjectType {
    Track,
    Via,
    Pad,
    Component,
    Keepout,
    BoardOutline,
    Polygon,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpatialObject {
    pub id: ObjectId,
    pub bbox: BoundingBox,
    pub object_type: SpatialObjectType,
    pub net_id: Option<NetId>,
    pub layer: LayerId,
}

/// Dynamic Spatial Index using hierarchical bounding boxes.
#[derive(Debug, Clone, Default)]
pub struct SpatialIndex {
    pub objects: Vec<SpatialObject>,
}

impl SpatialIndex {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Insert an object into the spatial index.
    pub fn insert(&mut self, object: SpatialObject) {
        self.objects.push(object);
    }

    /// Build spatial index from a PCB board definition.
    pub fn build(board: &PcbBoard) -> Self {
        let mut index = Self::new();

        // 1. Tracks / Segments
        for seg in &board.segments {
            let p1 = Point2D::from_mm(seg.start.x, seg.start.y);
            let p2 = Point2D::from_mm(seg.end.x, seg.end.y);
            let half_w = (seg.width * 500.0).round() as i64;
            let raw_bbox = BoundingBox::from_points(&[p1, p2]);
            let bbox = raw_bbox.expand(half_w);

            index.insert(SpatialObject {
                id: seg.uuid,
                bbox,
                object_type: SpatialObjectType::Track,
                net_id: Some(seg.net),
                layer: 0,
            });
        }

        // 2. Vias
        for via in &board.vias {
            let center = Point2D::from_mm(via.position.x, via.position.y);
            let radius = (via.diameter * 500.0).round() as i64;
            let bbox = BoundingBox::from_center_radius(center, radius);

            index.insert(SpatialObject {
                id: via.uuid,
                bbox,
                object_type: SpatialObjectType::Via,
                net_id: Some(via.net),
                layer: 0,
            });
        }

        // 3. Component Footprints and Pads
        for fp in &board.footprints {
            let fp_center = Point2D::from_mm(fp.position.x, fp.position.y);
            for pad in &fp.pads {
                let pad_pos = Point2D::from_mm(
                    fp.position.x + pad.position.x,
                    fp.position.y + pad.position.y,
                );
                let half_w = (pad.size.x * 500.0).round() as i64;
                let half_h = (pad.size.y * 500.0).round() as i64;
                let bbox = BoundingBox::new(
                    Point2D::new(pad_pos.x - half_w, pad_pos.y - half_h),
                    Point2D::new(pad_pos.x + half_w, pad_pos.y + half_h),
                );

                index.insert(SpatialObject {
                    id: pad.uuid,
                    bbox,
                    object_type: SpatialObjectType::Pad,
                    net_id: pad.net.as_ref().map(|n| n.number),
                    layer: 0,
                });
            }

            // Component boundary
            let fp_bbox = BoundingBox::from_center_radius(fp_center, 5000);
            index.insert(SpatialObject {
                id: fp.uuid,
                bbox: fp_bbox,
                object_type: SpatialObjectType::Component,
                net_id: None,
                layer: 0,
            });
        }

        index
    }

    /// Query objects whose bounding box intersects `bbox`.
    pub fn query_bbox(&self, bbox: &BoundingBox) -> Vec<&SpatialObject> {
        self.objects
            .iter()
            .filter(|obj| obj.bbox.intersects(bbox))
            .collect()
    }

    /// Query objects within a radius from center point.
    pub fn query_radius(&self, center: Point2D, radius: Microns) -> Vec<&SpatialObject> {
        let bbox = BoundingBox::from_center_radius(center, radius);
        self.objects
            .iter()
            .filter(|obj| {
                obj.bbox.intersects(&bbox) && obj.bbox.center().distance_to(center) <= radius
            })
            .collect()
    }

    /// Find nearest object to a point.
    pub fn nearest(&self, point: Point2D) -> Option<&SpatialObject> {
        self.objects
            .iter()
            .min_by_key(|obj| obj.bbox.center().distance_squared(point))
    }

    /// Check collision with bounding box, optionally excluding specified net IDs.
    pub fn check_collision(
        &self,
        bbox: &BoundingBox,
        exclude_nets: &[NetId],
    ) -> Vec<&SpatialObject> {
        self.objects
            .iter()
            .filter(|obj| {
                if !obj.bbox.intersects(bbox) {
                    return false;
                }
                if let Some(net_id) = obj.net_id {
                    !exclude_nets.contains(&net_id)
                } else {
                    true
                }
            })
            .collect()
    }

    /// Calculate minimum distance between two spatial objects.
    pub fn min_distance(&self, obj1: &SpatialObject, obj2: &SpatialObject) -> Microns {
        obj1.bbox.distance_to(&obj2.bbox)
    }

    /// Total count of indexed objects.
    pub fn count(&self) -> usize {
        self.objects.len()
    }
}
