# manager

## Classs

- [ConstraintManager](ConstraintManager.md) — Central constraint repository that resolves design rules hierarchically.
- [RuleConfigFile](RuleConfigFile.md) — Top-level schema for `rules.toml` configuration files.

## Functions

- [add_rule](add_rule.md) — Add a design rule to the manager.
- [add_rule](add_rule_1.md) — Add a design rule to the manager.
- [default_version](default_version.md)
- [from_file](from_file.md) — Load constraints from a `rules.toml` file on disk.
- [from_file](from_file_1.md) — Load constraints from a `rules.toml` file on disk.
- [from_toml_str](from_toml_str.md) — Parse constraints from a TOML configuration string.
- [from_toml_str](from_toml_str_1.md) — Parse constraints from a TOML configuration string.
- [new](new.md)
- [new](new_1.md)
- [resolve_clearance_rule](resolve_clearance_rule.md) — Resolve the most specific [`ClearanceRule`] for a given net, net class, and room.
- [resolve_clearance_rule](resolve_clearance_rule_1.md) — Resolve the most specific [`ClearanceRule`] for a given net, net class, and room.
- [resolve_component_clearance_rule](resolve_component_clearance_rule.md) — Resolve the most specific [`ComponentClearanceRule`] for a given component/room.
- [resolve_component_clearance_rule](resolve_component_clearance_rule_1.md) — Resolve the most specific [`ComponentClearanceRule`] for a given component/room.
- [resolve_diff_pair_phase_rule](resolve_diff_pair_phase_rule.md) — Resolve [`DiffPairPhaseRule`] for a given net class.
- [resolve_diff_pair_phase_rule](resolve_diff_pair_phase_rule_1.md) — Resolve [`DiffPairPhaseRule`] for a given net class.
- [resolve_high_speed_rule](resolve_high_speed_rule.md) — Resolve high-speed routing rule for a given net class.
- [resolve_high_speed_rule](resolve_high_speed_rule_1.md) — Resolve high-speed routing rule for a given net class.
- [resolve_net_antenna_rule](resolve_net_antenna_rule.md) — Resolve the most specific [`NetAntennaRule`] for a given net, net class, and room.
- [resolve_net_antenna_rule](resolve_net_antenna_rule_1.md) — Resolve the most specific [`NetAntennaRule`] for a given net, net class, and room.
- [resolve_polygon_connect_rule](resolve_polygon_connect_rule.md) — Resolve the most specific [`PolygonConnectRule`] for a given net, net class, and room.
- [resolve_polygon_connect_rule](resolve_polygon_connect_rule_1.md) — Resolve the most specific [`PolygonConnectRule`] for a given net, net class, and room.
- [resolve_return_path_rule](resolve_return_path_rule.md) — Resolve [`ReturnPathRule`] for a given net class.
- [resolve_return_path_rule](resolve_return_path_rule_1.md) — Resolve [`ReturnPathRule`] for a given net class.
- [resolve_routing_layer_rule](resolve_routing_layer_rule.md) — Resolve the most specific [`RoutingLayerRule`] for a given net, net class, and room.
- [resolve_routing_layer_rule](resolve_routing_layer_rule_1.md) — Resolve the most specific [`RoutingLayerRule`] for a given net, net class, and room.
- [resolve_silkscreen_rule](resolve_silkscreen_rule.md) — Resolve the most specific [`SilkscreenRule`] for a given net, net class, and room.
- [resolve_silkscreen_rule](resolve_silkscreen_rule_1.md) — Resolve the most specific [`SilkscreenRule`] for a given net, net class, and room.
- [resolve_solder_mask_rule](resolve_solder_mask_rule.md) — Resolve the most specific [`SolderMaskRule`] for a given net, net class, and room.
- [resolve_solder_mask_rule](resolve_solder_mask_rule_1.md) — Resolve the most specific [`SolderMaskRule`] for a given net, net class, and room.
- [resolve_via_style_rule](resolve_via_style_rule.md) — Resolve the most specific [`ViaStyleRule`] for a given net, net class, and room.
- [resolve_via_style_rule](resolve_via_style_rule_1.md) — Resolve the most specific [`ViaStyleRule`] for a given net, net class, and room.
- [resolve_width_rule](resolve_width_rule.md) — Resolve the most specific [`WidthRule`] that applies to a given net, net class, and room.
- [resolve_width_rule](resolve_width_rule_1.md) — Resolve the most specific [`WidthRule`] that applies to a given net, net class, and room.
- [save_to_file](save_to_file.md) — Save constraints to a `rules.toml` file on disk.
- [save_to_file](save_to_file_1.md) — Save constraints to a `rules.toml` file on disk.
- [standard_default](standard_default.md) — Standard baseline rules for typical 2-layer or 4-layer PCB fabrication
- [standard_default](standard_default_1.md) — Standard baseline rules for typical 2-layer or 4-layer PCB fabrication
- [to_toml_string](to_toml_string.md) — Serialize constraints to a formatted TOML string.
- [to_toml_string](to_toml_string_1.md) — Serialize constraints to a formatted TOML string.
- [validate_antenna_length](validate_antenna_length.md) — Validate that a net's dangling trace stub length does not exceed antenna threshold.
- [validate_antenna_length](validate_antenna_length_1.md) — Validate that a net's dangling trace stub length does not exceed antenna threshold.
- [validate_clearance](validate_clearance.md) — Validate electrical clearance distance between two nets or objects.
- [validate_clearance](validate_clearance_1.md) — Validate electrical clearance distance between two nets or objects.
- [validate_component_clearance](validate_component_clearance.md) — Validate 3D component horizontal (X/Y) or vertical (Z) clearance.
- [validate_component_clearance](validate_component_clearance_1.md) — Validate 3D component horizontal (X/Y) or vertical (Z) clearance.
- [validate_diff_pair_phase](validate_diff_pair_phase.md) — Validate differential pair intra-pair phase skew or inter-pair bus skew.
- [validate_diff_pair_phase](validate_diff_pair_phase_1.md) — Validate differential pair intra-pair phase skew or inter-pair bus skew.
- [validate_return_path](validate_return_path.md) — Validate unbroken reference return path for high-speed net.
- [validate_return_path](validate_return_path_1.md) — Validate unbroken reference return path for high-speed net.
- [validate_routing_layer](validate_routing_layer.md) — Validate that a routed trace is placed on an authorized copper layer.
- [validate_routing_layer](validate_routing_layer_1.md) — Validate that a routed trace is placed on an authorized copper layer.
- [validate_silkscreen_clearance](validate_silkscreen_clearance.md) — Validate silkscreen clearance to solder mask or adjacent silkscreen.
- [validate_silkscreen_clearance](validate_silkscreen_clearance_1.md) — Validate silkscreen clearance to solder mask or adjacent silkscreen.
- [validate_solder_mask_sliver](validate_solder_mask_sliver.md) — Validate that a solder mask bridge/sliver meets the minimum width requirement.
- [validate_solder_mask_sliver](validate_solder_mask_sliver_1.md) — Validate that a solder mask bridge/sliver meets the minimum width requirement.
- [validate_trace_width](validate_trace_width.md) — Validate that a routed trace width conforms to the hierarchical width rule.
- [validate_trace_width](validate_trace_width_1.md) — Validate that a routed trace width conforms to the hierarchical width rule.
