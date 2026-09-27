---
okf_version: "0.2"
type: Module
title: manager
description: Hierarchical Constraint Manager and DRC rule evaluation engine.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager
language: rust
---

# manager

Hierarchical Constraint Manager and DRC rule evaluation engine.

## Docstring

Hierarchical Constraint Manager and DRC rule evaluation engine.

## Relationships

| Type | Target |
|------|--------|
| related | [ConstraintManager](/crates/oxide-rules/src/manager/ConstraintManager.md) |
| related | [new](/crates/oxide-rules/src/manager/new.md) |
| related | [add_rule](/crates/oxide-rules/src/manager/add_rule.md) |
| related | [standard_default](/crates/oxide-rules/src/manager/standard_default.md) |
| related | [resolve_width_rule](/crates/oxide-rules/src/manager/resolve_width_rule.md) |
| related | [resolve_clearance_rule](/crates/oxide-rules/src/manager/resolve_clearance_rule.md) |
| related | [resolve_high_speed_rule](/crates/oxide-rules/src/manager/resolve_high_speed_rule.md) |
| related | [validate_trace_width](/crates/oxide-rules/src/manager/validate_trace_width.md) |
| related | [validate_clearance](/crates/oxide-rules/src/manager/validate_clearance.md) |
| related | [resolve_via_style_rule](/crates/oxide-rules/src/manager/resolve_via_style_rule.md) |
| related | [resolve_polygon_connect_rule](/crates/oxide-rules/src/manager/resolve_polygon_connect_rule.md) |
| related | [resolve_solder_mask_rule](/crates/oxide-rules/src/manager/resolve_solder_mask_rule.md) |
| related | [resolve_silkscreen_rule](/crates/oxide-rules/src/manager/resolve_silkscreen_rule.md) |
| related | [resolve_net_antenna_rule](/crates/oxide-rules/src/manager/resolve_net_antenna_rule.md) |
| related | [resolve_return_path_rule](/crates/oxide-rules/src/manager/resolve_return_path_rule.md) |
| related | [validate_solder_mask_sliver](/crates/oxide-rules/src/manager/validate_solder_mask_sliver.md) |
| related | [validate_silkscreen_clearance](/crates/oxide-rules/src/manager/validate_silkscreen_clearance.md) |
| related | [validate_antenna_length](/crates/oxide-rules/src/manager/validate_antenna_length.md) |
| related | [validate_return_path](/crates/oxide-rules/src/manager/validate_return_path.md) |
| related | [resolve_component_clearance_rule](/crates/oxide-rules/src/manager/resolve_component_clearance_rule.md) |
| related | [resolve_routing_layer_rule](/crates/oxide-rules/src/manager/resolve_routing_layer_rule.md) |
| related | [resolve_diff_pair_phase_rule](/crates/oxide-rules/src/manager/resolve_diff_pair_phase_rule.md) |
| related | [validate_component_clearance](/crates/oxide-rules/src/manager/validate_component_clearance.md) |
| related | [validate_routing_layer](/crates/oxide-rules/src/manager/validate_routing_layer.md) |
| related | [validate_diff_pair_phase](/crates/oxide-rules/src/manager/validate_diff_pair_phase.md) |
| related | [from_toml_str](/crates/oxide-rules/src/manager/from_toml_str.md) |
| related | [to_toml_string](/crates/oxide-rules/src/manager/to_toml_string.md) |
| related | [from_file](/crates/oxide-rules/src/manager/from_file.md) |
| related | [save_to_file](/crates/oxide-rules/src/manager/save_to_file.md) |
| related | [new](/crates/oxide-rules/src/manager/new.md) |
| related | [add_rule](/crates/oxide-rules/src/manager/add_rule.md) |
| related | [standard_default](/crates/oxide-rules/src/manager/standard_default.md) |
| related | [resolve_width_rule](/crates/oxide-rules/src/manager/resolve_width_rule.md) |
| related | [resolve_clearance_rule](/crates/oxide-rules/src/manager/resolve_clearance_rule.md) |
| related | [resolve_high_speed_rule](/crates/oxide-rules/src/manager/resolve_high_speed_rule.md) |
| related | [validate_trace_width](/crates/oxide-rules/src/manager/validate_trace_width.md) |
| related | [validate_clearance](/crates/oxide-rules/src/manager/validate_clearance.md) |
| related | [resolve_via_style_rule](/crates/oxide-rules/src/manager/resolve_via_style_rule.md) |
| related | [resolve_polygon_connect_rule](/crates/oxide-rules/src/manager/resolve_polygon_connect_rule.md) |
| related | [resolve_solder_mask_rule](/crates/oxide-rules/src/manager/resolve_solder_mask_rule.md) |
| related | [resolve_silkscreen_rule](/crates/oxide-rules/src/manager/resolve_silkscreen_rule.md) |
| related | [resolve_net_antenna_rule](/crates/oxide-rules/src/manager/resolve_net_antenna_rule.md) |
| related | [resolve_return_path_rule](/crates/oxide-rules/src/manager/resolve_return_path_rule.md) |
| related | [validate_solder_mask_sliver](/crates/oxide-rules/src/manager/validate_solder_mask_sliver.md) |
| related | [validate_silkscreen_clearance](/crates/oxide-rules/src/manager/validate_silkscreen_clearance.md) |
| related | [validate_antenna_length](/crates/oxide-rules/src/manager/validate_antenna_length.md) |
| related | [validate_return_path](/crates/oxide-rules/src/manager/validate_return_path.md) |
| related | [resolve_component_clearance_rule](/crates/oxide-rules/src/manager/resolve_component_clearance_rule.md) |
| related | [resolve_routing_layer_rule](/crates/oxide-rules/src/manager/resolve_routing_layer_rule.md) |
| related | [resolve_diff_pair_phase_rule](/crates/oxide-rules/src/manager/resolve_diff_pair_phase_rule.md) |
| related | [validate_component_clearance](/crates/oxide-rules/src/manager/validate_component_clearance.md) |
| related | [validate_routing_layer](/crates/oxide-rules/src/manager/validate_routing_layer.md) |
| related | [validate_diff_pair_phase](/crates/oxide-rules/src/manager/validate_diff_pair_phase.md) |
| related | [from_toml_str](/crates/oxide-rules/src/manager/from_toml_str.md) |
| related | [to_toml_string](/crates/oxide-rules/src/manager/to_toml_string.md) |
| related | [from_file](/crates/oxide-rules/src/manager/from_file.md) |
| related | [save_to_file](/crates/oxide-rules/src/manager/save_to_file.md) |
| related | [RuleConfigFile](/crates/oxide-rules/src/manager/RuleConfigFile.md) |
| related | [default_version](/crates/oxide-rules/src/manager/default_version.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
