---
okf_version: "0.2"
type: Function
title: dirty_flags_for_events
resource: crates/oxide-renderer/src/pcb/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/src/pcb/mod/dirty_flags_for_events
language: rust
---

# dirty_flags_for_events

## Signature

```rust
pub fn dirty_flags_for_events(events: &[PcbAppEvent]) -> DirtyFlags
```

## Visibility

- `pub`

## Source
Lines 220–228 in `crates/oxide-renderer/src/pcb/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb](/crates/oxide-renderer/src/pcb/mod.md) |
| calls | [dirty_flags_for_event](/crates/oxide-renderer/src/pcb/mod/dirty_flags_for_event.md) |
| called_by | [apply_pcb_renderer_dirty_hint](/crates/oxide-app/src/app/pcb_dirty_adapter/apply_pcb_renderer_dirty_hint.md) |
| called_by | [pcb_event_mapping_routes_to_expected_dirty_flags](/crates/oxide-renderer/src/pcb/mod/pcb_event_mapping_routes_to_expected_dirty_flags.md) |
| called_by | [camera_only_event_does_not_request_geometry_uploads](/crates/oxide-renderer/tests/pcb_dirty_event_integration/camera_only_event_does_not_request_geometry_uploads.md) |
| called_by | [pcb_event_flow_updates_only_expected_scene_families](/crates/oxide-renderer/tests/pcb_dirty_event_integration/pcb_event_flow_updates_only_expected_scene_families.md) |
