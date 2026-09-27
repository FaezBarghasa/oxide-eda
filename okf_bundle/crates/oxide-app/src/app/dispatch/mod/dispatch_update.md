---
okf_version: "0.2"
type: Function
title: dispatch_update
resource: crates/oxide-app/src/app/dispatch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/mod/dispatch_update
language: rust
---

# dispatch_update

## Signature

```rust
impl Oxide { pub(crate) fn dispatch_update(&mut self, message: Message) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Source
Lines 17–243 in `crates/oxide-app/src/app/dispatch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dispatch](/crates/oxide-app/src/app/dispatch/mod.md) |
| calls | [write_first_run_tour_dismissed](/crates/oxide-app/src/fonts/misc/write_first_run_tour_dismissed.md) |
| calls | [hit_test_polygon](/crates/oxide-app/src/schematic_runtime/hit_test/hit_test_polygon.md) |
| calls | [passes_filter](/crates/oxide-app/src/app/handlers/selection_workflow/passes_filter.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
