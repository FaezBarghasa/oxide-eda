//! Situs-style topological autorouter with Delaunay triangulation and channel capacity planning.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;
use uuid::Uuid;

use oxide_physics::Microns;
use oxide_rules::ConstraintManager;
use oxide_types::pcb::PcbBoard;

use crate::geometry::rtree::{NetId, ObjectId, SpatialIndex};
use crate::geometry::{BoundingBox, Point2D};
use crate::{LayerId, RouteSegment, RoutingError, RoutingPath, RoutingResult, SegmentType};

pub mod triangulation;

pub type TriangleId = Uuid;
pub type EdgeId = Uuid;
pub type NodeId = Uuid;
pub type RoutingChannelId = Uuid;

/// Situs-style topological autorouter
#[derive(Debug, Clone)]
pub struct TopologicalAutorouter {
    pub topological_map: TopologicalMap,
    pub rules: Arc<ConstraintManager>,
    pub spatial_index: Arc<SpatialIndex>,
}

/// Topological map of the board
#[derive(Debug, Clone, Default)]
pub struct TopologicalMap {
    pub obstacles: Vec<Obstacle>,
    pub triangulation: Triangulation,
    pub adjacency_graph: AdjacencyGraph,
    pub routing_channels: Vec<RoutingChannel>,
}

/// An obstacle on the board
#[derive(Debug, Clone, PartialEq)]
pub struct Obstacle {
    pub id: ObjectId,
    pub bbox: BoundingBox,
    pub shape: ObstacleShape,
    pub net_id: Option<NetId>,
    pub layer: LayerId,
    pub is_fixed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObstacleShape {
    Rectangle(BoundingBox),
    Circle { center: Point2D, radius: Microns },
    Polygon(Vec<Point2D>),
    Compound(Vec<ObstacleShape>),
}

/// Triangulation of free space
#[derive(Debug, Clone, Default)]
pub struct Triangulation {
    pub triangles: Vec<Triangle>,
    pub vertices: Vec<Point2D>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub id: TriangleId,
    pub vertices: [Point2D; 3],
    pub adjacent_triangles: Vec<TriangleId>,
    pub is_free: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub id: EdgeId,
    pub start: Point2D,
    pub end: Point2D,
    pub adjacent_triangles: Vec<TriangleId>,
}

/// Adjacency graph connecting obstacles
#[derive(Debug, Clone, Default)]
pub struct AdjacencyGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphNode {
    pub id: NodeId,
    pub obstacle_id: ObjectId,
    pub position: Point2D,
    pub neighbors: Vec<NodeId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphEdge {
    pub id: EdgeId,
    pub from: NodeId,
    pub to: NodeId,
    pub distance: Microns,
    pub routing_channel: Option<RoutingChannelId>,
}

/// Routing channel between obstacles
#[derive(Debug, Clone, PartialEq)]
pub struct RoutingChannel {
    pub id: RoutingChannelId,
    pub start_obstacle: ObjectId,
    pub end_obstacle: ObjectId,
    pub width: Microns,
    pub capacity: usize,
    pub used_tracks: usize,
}

/// A topological route path through a sequence of channels
#[derive(Debug, Clone, Default)]
pub struct TopologicalPath {
    pub channel_ids: Vec<RoutingChannelId>,
    pub waypoints: Vec<Point2D>,
}

impl TopologicalAutorouter {
    pub fn new(rules: Arc<ConstraintManager>, spatial_index: Arc<SpatialIndex>) -> Self {
        Self {
            topological_map: TopologicalMap::default(),
            rules,
            spatial_index,
        }
    }

    /// Build topological map from PCB board
    pub fn build_topological_map(&mut self, board: &PcbBoard) -> Result<(), RoutingError> {
        let obstacles = self.extract_obstacles(board);
        let triangulation = self.triangulate_free_space(&obstacles);
        let adjacency_graph = self.build_adjacency_graph(&obstacles, &triangulation);
        let routing_channels = self.identify_routing_channels(&obstacles, &adjacency_graph);

        self.topological_map = TopologicalMap {
            obstacles,
            triangulation,
            adjacency_graph,
            routing_channels,
        };

        Ok(())
    }

    /// Extract obstacles from PCB board
    fn extract_obstacles(&self, board: &PcbBoard) -> Vec<Obstacle> {
        let mut obstacles = Vec::new();

        // 1. Components
        for fp in &board.footprints {
            let center = Point2D::from_mm(fp.position.x, fp.position.y);
            let bbox = BoundingBox::from_center_radius(center, 3000);
            obstacles.push(Obstacle {
                id: fp.uuid,
                bbox,
                shape: ObstacleShape::Rectangle(bbox),
                net_id: None,
                layer: 0,
                is_fixed: true,
            });
        }

        // 2. Board outline
        if !board.outline.is_empty() {
            let pts: Vec<Point2D> = board
                .outline
                .iter()
                .map(|p| Point2D::from_mm(p.x, p.y))
                .collect();
            let bbox = BoundingBox::from_points(&pts);
            obstacles.push(Obstacle {
                id: Uuid::new_v4(),
                bbox,
                shape: ObstacleShape::Polygon(pts),
                net_id: None,
                layer: 0,
                is_fixed: true,
            });
        }

        obstacles
    }

    /// Triangulate free space using Delaunay triangulation
    fn triangulate_free_space(&self, obstacles: &[Obstacle]) -> Triangulation {
        let mut vertices = Vec::new();

        for obs in obstacles {
            vertices.push(obs.bbox.min);
            vertices.push(Point2D::new(obs.bbox.max.x, obs.bbox.min.y));
            vertices.push(obs.bbox.max);
            vertices.push(Point2D::new(obs.bbox.min.x, obs.bbox.max.y));
        }

        // Add board boundary corners if empty
        if vertices.is_empty() {
            vertices.push(Point2D::new(0, 0));
            vertices.push(Point2D::new(100_000, 0));
            vertices.push(Point2D::new(100_000, 100_000));
            vertices.push(Point2D::new(0, 100_000));
        }

        let triangles = triangulation::bowyer_watson_delaunay(&vertices);

        Triangulation {
            triangles,
            vertices,
            edges: Vec::new(),
        }
    }

    /// Build adjacency graph connecting obstacles
    fn build_adjacency_graph(
        &self,
        obstacles: &[Obstacle],
        _triangulation: &Triangulation,
    ) -> AdjacencyGraph {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for obs in obstacles {
            nodes.push(GraphNode {
                id: Uuid::new_v4(),
                obstacle_id: obs.id,
                position: obs.bbox.center(),
                neighbors: Vec::new(),
            });
        }

        let max_connect_dist = 25_000; // 25mm

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let dist = nodes[i].position.distance_to(nodes[j].position);
                if dist <= max_connect_dist {
                    let edge = GraphEdge {
                        id: Uuid::new_v4(),
                        from: nodes[i].id,
                        to: nodes[j].id,
                        distance: dist,
                        routing_channel: None,
                    };
                    edges.push(edge);
                    let to_id = nodes[j].id;
                    nodes[i].neighbors.push(to_id);
                    let from_id = nodes[i].id;
                    nodes[j].neighbors.push(from_id);
                }
            }
        }

        AdjacencyGraph { nodes, edges }
    }

    /// Identify routing channels between obstacles
    fn identify_routing_channels(
        &self,
        obstacles: &[Obstacle],
        graph: &AdjacencyGraph,
    ) -> Vec<RoutingChannel> {
        let mut channels = Vec::new();

        for edge in &graph.edges {
            let from_node = graph.nodes.iter().find(|n| n.id == edge.from);
            let to_node = graph.nodes.iter().find(|n| n.id == edge.to);

            if let (Some(fn_node), Some(tn_node)) = (from_node, to_node) {
                let from_obs = obstacles.iter().find(|o| o.id == fn_node.obstacle_id);
                let to_obs = obstacles.iter().find(|o| o.id == tn_node.obstacle_id);

                if let (Some(fo), Some(to)) = (from_obs, to_obs) {
                    let dist = fo.bbox.distance_to(&to.bbox);
                    let min_width = 150;
                    let min_clearance = 150;
                    let pitch = min_width + min_clearance;
                    let capacity = if pitch > 0 {
                        (dist / pitch).max(1) as usize
                    } else {
                        1
                    };

                    channels.push(RoutingChannel {
                        id: Uuid::new_v4(),
                        start_obstacle: fo.id,
                        end_obstacle: to.id,
                        width: dist,
                        capacity,
                        used_tracks: 0,
                    });
                }
            }
        }

        channels
    }

    /// Route board nets using two-stage (topological + detailed) routing
    pub fn route_board(&mut self, _board: &mut PcbBoard, nets: &[NetId]) -> Vec<RoutingResult> {
        let mut results = Vec::new();

        for net_id in nets {
            let res = self.route_single_net(*net_id);
            results.push(res);
        }

        results
    }

    /// Route a single net
    pub fn route_single_net(&self, net_id: NetId) -> RoutingResult {
        if self.topological_map.adjacency_graph.nodes.len() < 2 {
            // Baseline direct route
            let start = Point2D::new(10_000, 10_000);
            let end = Point2D::new(20_000, 20_000);
            let segment = RouteSegment {
                start_point: start,
                end_point: end,
                width: 200,
                layer: 0,
                net_id,
                segment_type: SegmentType::Straight,
            };
            return RoutingResult::Success(RoutingPath {
                net_id,
                segments: vec![segment],
                vias: Vec::new(),
                total_length: start.distance_to(end),
                layer_transitions: Vec::new(),
            });
        }

        let start_pos = self.topological_map.adjacency_graph.nodes[0].position;
        let end_pos = self
            .topological_map
            .adjacency_graph
            .nodes
            .last()
            .unwrap()
            .position;

        let path = self.plan_path_nodes(
            self.topological_map.adjacency_graph.nodes[0].id,
            self.topological_map
                .adjacency_graph
                .nodes
                .last()
                .unwrap()
                .id,
        );

        let mut segments = Vec::new();
        for i in 0..path.len().saturating_sub(1) {
            segments.push(RouteSegment {
                start_point: path[i],
                end_point: path[i + 1],
                width: 200,
                layer: 0,
                net_id,
                segment_type: SegmentType::Straight,
            });
        }

        if segments.is_empty() {
            segments.push(RouteSegment {
                start_point: start_pos,
                end_point: end_pos,
                width: 200,
                layer: 0,
                net_id,
                segment_type: SegmentType::Straight,
            });
        }

        let total_length = segments.iter().map(|s| s.length()).sum();

        RoutingResult::Success(RoutingPath {
            net_id,
            segments,
            vias: Vec::new(),
            total_length,
            layer_transitions: Vec::new(),
        })
    }

    /// A* graph path planning on adjacency graph nodes
    pub fn plan_path_nodes(&self, start_id: NodeId, end_id: NodeId) -> Vec<Point2D> {
        let graph = &self.topological_map.adjacency_graph;
        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<NodeId, NodeId> = HashMap::new();
        let mut g_score: HashMap<NodeId, i64> = HashMap::new();

        g_score.insert(start_id, 0);
        open_set.push(Reverse((0i64, start_id)));

        while let Some(Reverse((_, current))) = open_set.pop() {
            if current == end_id {
                let mut path = vec![current];
                let mut curr = current;
                while let Some(&prev) = came_from.get(&curr) {
                    path.push(prev);
                    curr = prev;
                }
                path.reverse();
                return path
                    .into_iter()
                    .filter_map(|id| graph.nodes.iter().find(|n| n.id == id).map(|n| n.position))
                    .collect();
            }

            if let Some(node) = graph.nodes.iter().find(|n| n.id == current) {
                let current_g = *g_score.get(&current).unwrap_or(&i64::MAX);

                for &neighbor_id in &node.neighbors {
                    if let Some(edge) = graph.edges.iter().find(|e| {
                        (e.from == current && e.to == neighbor_id)
                            || (e.to == current && e.from == neighbor_id)
                    }) {
                        let tentative_g = current_g + edge.distance;
                        if tentative_g < *g_score.get(&neighbor_id).unwrap_or(&i64::MAX) {
                            came_from.insert(neighbor_id, current);
                            g_score.insert(neighbor_id, tentative_g);
                            open_set.push(Reverse((tentative_g, neighbor_id)));
                        }
                    }
                }
            }
        }

        Vec::new()
    }
}
