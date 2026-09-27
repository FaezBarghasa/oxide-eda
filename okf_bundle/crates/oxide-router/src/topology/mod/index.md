# mod

## Classs

- [AdjacencyGraph](AdjacencyGraph.md) — Adjacency graph connecting obstacles
- [Edge](Edge.md) — [derive(Debug, Clone, PartialEq)]
- [GraphEdge](GraphEdge.md) — [derive(Debug, Clone, PartialEq)]
- [GraphNode](GraphNode.md) — [derive(Debug, Clone, PartialEq)]
- [Obstacle](Obstacle.md) — An obstacle on the board
- [ObstacleShape](ObstacleShape.md) — [derive(Debug, Clone, PartialEq)]
- [RoutingChannel](RoutingChannel.md) — Routing channel between obstacles
- [TopologicalAutorouter](TopologicalAutorouter.md) — Situs-style topological autorouter
- [TopologicalMap](TopologicalMap.md) — Topological map of the board
- [TopologicalPath](TopologicalPath.md) — A topological route path through a sequence of channels
- [Triangle](Triangle.md) — [derive(Debug, Clone, PartialEq)]
- [Triangulation](Triangulation.md) — Triangulation of free space

## Functions

- [build_adjacency_graph](build_adjacency_graph.md) — Build adjacency graph connecting obstacles
- [build_adjacency_graph](build_adjacency_graph_1.md) — Build adjacency graph connecting obstacles
- [build_topological_map](build_topological_map.md) — Build topological map from PCB board
- [build_topological_map](build_topological_map_1.md) — Build topological map from PCB board
- [extract_obstacles](extract_obstacles.md) — Extract obstacles from PCB board
- [extract_obstacles](extract_obstacles_1.md) — Extract obstacles from PCB board
- [identify_routing_channels](identify_routing_channels.md) — Identify routing channels between obstacles
- [identify_routing_channels](identify_routing_channels_1.md) — Identify routing channels between obstacles
- [new](new.md)
- [new](new_1.md)
- [plan_path_nodes](plan_path_nodes.md) — A* graph path planning on adjacency graph nodes
- [plan_path_nodes](plan_path_nodes_1.md) — A* graph path planning on adjacency graph nodes
- [route_board](route_board.md) — Route board nets using two-stage (topological + detailed) routing
- [route_board](route_board_1.md) — Route board nets using two-stage (topological + detailed) routing
- [route_single_net](route_single_net.md) — Route a single net
- [route_single_net](route_single_net_1.md) — Route a single net
- [triangulate_free_space](triangulate_free_space.md) — Triangulate free space using Delaunay triangulation
- [triangulate_free_space](triangulate_free_space_1.md) — Triangulate free space using Delaunay triangulation
