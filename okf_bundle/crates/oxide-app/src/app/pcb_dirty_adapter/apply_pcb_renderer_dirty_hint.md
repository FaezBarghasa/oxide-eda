---
okf_version: "0.2"
type: Function
title: apply_pcb_renderer_dirty_hint
resource: crates/oxide-app/src/app/pcb_dirty_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/pcb_dirty_adapter/apply_pcb_renderer_dirty_hint
language: rust
---

# apply_pcb_renderer_dirty_hint

## Signature

```rust
impl Oxide { pub(crate) fn apply_pcb_renderer_dirty_hint(&mut self, message: &Message) }
```

## Visibility

- `pub(crate)`

## Source
Lines 47–70 in `crates/oxide-app/src/app/pcb_dirty_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_dirty_adapter](/crates/oxide-app/src/app/pcb_dirty_adapter.md) |
| calls | [pcb_renderer_events_for_message](/crates/oxide-app/src/app/pcb_dirty_adapter/pcb_renderer_events_for_message.md) |
| calls | [dirty_flags_for_events](/crates/oxide-renderer/src/pcb/mod/dirty_flags_for_events.md) |
