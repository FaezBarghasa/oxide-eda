---
okf_version: "0.2"
type: Function
title: dirty_flags_for_event
resource: crates/oxide-renderer/src/pcb/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/src/pcb/mod/dirty_flags_for_event
language: rust
---

# dirty_flags_for_event

## Signature

```rust
pub fn dirty_flags_for_event(event: PcbAppEvent) -> DirtyFlags
```

## Visibility

- `pub`

## Source
Lines 216–218 in `crates/oxide-renderer/src/pcb/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb](/crates/oxide-renderer/src/pcb/mod.md) |
| calls | [dirty_flags_for_families](/crates/oxide-renderer/src/pcb/mod/dirty_flags_for_families.md) |
| calls | [families_for_event](/crates/oxide-renderer/src/pcb/mod/families_for_event.md) |
| called_by | [dirty_flags_for_events](/crates/oxide-renderer/src/pcb/mod/dirty_flags_for_events.md) |
| called_by | [pcb_event_mapping_routes_to_expected_dirty_flags](/crates/oxide-renderer/src/pcb/mod/pcb_event_mapping_routes_to_expected_dirty_flags.md) |
