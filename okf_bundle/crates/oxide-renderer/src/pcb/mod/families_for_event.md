---
okf_version: "0.2"
type: Function
title: families_for_event
resource: crates/oxide-renderer/src/pcb/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/src/pcb/mod/families_for_event
language: rust
---

# families_for_event

## Signature

```rust
pub fn families_for_event(event: PcbAppEvent) -> &'static [PcbSliceFamily]
```

## Visibility

- `pub`

## Source
Lines 182–195 in `crates/oxide-renderer/src/pcb/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb](/crates/oxide-renderer/src/pcb/mod.md) |
| called_by | [dirty_flags_for_event](/crates/oxide-renderer/src/pcb/mod/dirty_flags_for_event.md) |
