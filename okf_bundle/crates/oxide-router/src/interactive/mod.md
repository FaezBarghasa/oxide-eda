---
okf_version: "0.2"
type: Module
title: interactive
description: "Interactive routing engine with weighted A* pathfinding, Push-and-Shove,"
resource: crates/oxide-router/src/interactive/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:13:12Z"
concept_id: crates/oxide-router/src/interactive/mod
language: rust
---

# interactive

Interactive routing engine with weighted A* pathfinding, Push-and-Shove,

## Docstring

Interactive routing engine with weighted A* pathfinding, Push-and-Shove,
Walk-Around, Hug-and-Push, Differential Pair, and Length Tuning modes.

## Relationships

| Type | Target |
|------|--------|
| related | [RoutingMode](/crates/oxide-router/src/interactive/mod/RoutingMode.md) |
| related | [InteractiveRouter](/crates/oxide-router/src/interactive/mod/InteractiveRouter.md) |
| related | [new](/crates/oxide-router/src/interactive/mod/new.md) |
| related | [start_routing](/crates/oxide-router/src/interactive/mod/start_routing.md) |
| related | [enable_length_tuning](/crates/oxide-router/src/interactive/mod/enable_length_tuning.md) |
| related | [hotkey_increase_amplitude](/crates/oxide-router/src/interactive/mod/hotkey_increase_amplitude.md) |
| related | [hotkey_decrease_amplitude](/crates/oxide-router/src/interactive/mod/hotkey_decrease_amplitude.md) |
| related | [hotkey_increase_pitch](/crates/oxide-router/src/interactive/mod/hotkey_increase_pitch.md) |
| related | [hotkey_decrease_pitch](/crates/oxide-router/src/interactive/mod/hotkey_decrease_pitch.md) |
| related | [hotkey_cycle_corner_style](/crates/oxide-router/src/interactive/mod/hotkey_cycle_corner_style.md) |
| related | [hotkey_drop_via_and_switch_layer](/crates/oxide-router/src/interactive/mod/hotkey_drop_via_and_switch_layer.md) |
| related | [on_mouse_move](/crates/oxide-router/src/interactive/mod/on_mouse_move.md) |
| related | [route_net](/crates/oxide-router/src/interactive/mod/route_net.md) |
| related | [direct_route](/crates/oxide-router/src/interactive/mod/direct_route.md) |
| related | [stop_at_obstacle_route](/crates/oxide-router/src/interactive/mod/stop_at_obstacle_route.md) |
| related | [walk_around_route](/crates/oxide-router/src/interactive/mod/walk_around_route.md) |
| related | [push_and_shove_with_displacements](/crates/oxide-router/src/interactive/mod/push_and_shove_with_displacements.md) |
| related | [push_and_shove_route](/crates/oxide-router/src/interactive/mod/push_and_shove_route.md) |
| related | [hug_and_push_route](/crates/oxide-router/src/interactive/mod/hug_and_push_route.md) |
| related | [differential_pair_route](/crates/oxide-router/src/interactive/mod/differential_pair_route.md) |
| related | [length_tuning_route](/crates/oxide-router/src/interactive/mod/length_tuning_route.md) |
| related | [generate_meander](/crates/oxide-router/src/interactive/mod/generate_meander.md) |
| related | [generate_meander_with_params](/crates/oxide-router/src/interactive/mod/generate_meander_with_params.md) |
| related | [get_track_width_for_net](/crates/oxide-router/src/interactive/mod/get_track_width_for_net.md) |
| related | [new](/crates/oxide-router/src/interactive/mod/new.md) |
| related | [start_routing](/crates/oxide-router/src/interactive/mod/start_routing.md) |
| related | [enable_length_tuning](/crates/oxide-router/src/interactive/mod/enable_length_tuning.md) |
| related | [hotkey_increase_amplitude](/crates/oxide-router/src/interactive/mod/hotkey_increase_amplitude.md) |
| related | [hotkey_decrease_amplitude](/crates/oxide-router/src/interactive/mod/hotkey_decrease_amplitude.md) |
| related | [hotkey_increase_pitch](/crates/oxide-router/src/interactive/mod/hotkey_increase_pitch.md) |
| related | [hotkey_decrease_pitch](/crates/oxide-router/src/interactive/mod/hotkey_decrease_pitch.md) |
| related | [hotkey_cycle_corner_style](/crates/oxide-router/src/interactive/mod/hotkey_cycle_corner_style.md) |
| related | [hotkey_drop_via_and_switch_layer](/crates/oxide-router/src/interactive/mod/hotkey_drop_via_and_switch_layer.md) |
| related | [on_mouse_move](/crates/oxide-router/src/interactive/mod/on_mouse_move.md) |
| related | [route_net](/crates/oxide-router/src/interactive/mod/route_net.md) |
| related | [direct_route](/crates/oxide-router/src/interactive/mod/direct_route.md) |
| related | [stop_at_obstacle_route](/crates/oxide-router/src/interactive/mod/stop_at_obstacle_route.md) |
| related | [walk_around_route](/crates/oxide-router/src/interactive/mod/walk_around_route.md) |
| related | [push_and_shove_with_displacements](/crates/oxide-router/src/interactive/mod/push_and_shove_with_displacements.md) |
| related | [push_and_shove_route](/crates/oxide-router/src/interactive/mod/push_and_shove_route.md) |
| related | [hug_and_push_route](/crates/oxide-router/src/interactive/mod/hug_and_push_route.md) |
| related | [differential_pair_route](/crates/oxide-router/src/interactive/mod/differential_pair_route.md) |
| related | [length_tuning_route](/crates/oxide-router/src/interactive/mod/length_tuning_route.md) |
| related | [generate_meander](/crates/oxide-router/src/interactive/mod/generate_meander.md) |
| related | [generate_meander_with_params](/crates/oxide-router/src/interactive/mod/generate_meander_with_params.md) |
| related | [get_track_width_for_net](/crates/oxide-router/src/interactive/mod/get_track_width_for_net.md) |
| related | [TuningConstraint](/crates/oxide-router/src/interactive/mod/TuningConstraint.md) |
| related | [TuningStyle](/crates/oxide-router/src/interactive/mod/TuningStyle.md) |
| related | [PathDeflection](/crates/oxide-router/src/interactive/mod/PathDeflection.md) |
| related | [TopologicalRouter](/crates/oxide-router/src/interactive/mod/TopologicalRouter.md) |
| related | [build_triangulation](/crates/oxide-router/src/interactive/mod/build_triangulation.md) |
| related | [evaluate_corridor](/crates/oxide-router/src/interactive/mod/evaluate_corridor.md) |
| related | [push_and_hug](/crates/oxide-router/src/interactive/mod/push_and_hug.md) |
| related | [apply_tuning](/crates/oxide-router/src/interactive/mod/apply_tuning.md) |
| related | [build_triangulation](/crates/oxide-router/src/interactive/mod/build_triangulation.md) |
| related | [evaluate_corridor](/crates/oxide-router/src/interactive/mod/evaluate_corridor.md) |
| related | [push_and_hug](/crates/oxide-router/src/interactive/mod/push_and_hug.md) |
| related | [apply_tuning](/crates/oxide-router/src/interactive/mod/apply_tuning.md) |
