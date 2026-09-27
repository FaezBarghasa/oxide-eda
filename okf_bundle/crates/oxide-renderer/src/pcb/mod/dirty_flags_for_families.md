---
okf_version: "0.2"
type: Function
title: dirty_flags_for_families
resource: crates/oxide-renderer/src/pcb/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/src/pcb/mod/dirty_flags_for_families
language: rust
---

# dirty_flags_for_families

## Signature

```rust
pub fn dirty_flags_for_families(families: &[PcbSliceFamily]) -> DirtyFlags
```

## Visibility

- `pub`

## Source
Lines 197–214 in `crates/oxide-renderer/src/pcb/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb](/crates/oxide-renderer/src/pcb/mod.md) |
| called_by | [dirty_flags_for_event](/crates/oxide-renderer/src/pcb/mod/dirty_flags_for_event.md) |
| called_by | [pcb_slice_dirty_mapping_resolves_expected_flags](/crates/oxide-renderer/src/pcb/mod/pcb_slice_dirty_mapping_resolves_expected_flags.md) |
