---
okf_version: "0.2"
type: Function
title: handle_toggle_custom_filter_preset_member
resource: crates/oxide-app/src/app/handlers/active_bar/filter_controls.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/active_bar/filter_controls/handle_toggle_custom_filter_preset_member
language: rust
---

# handle_toggle_custom_filter_preset_member

## Signature

```rust
impl Oxide { pub(crate) fn handle_toggle_custom_filter_preset_member(
        &mut self,
        idx: usize,
        filter: SelectionFilter,
    ) }
```

## Visibility

- `pub(crate)`

## Source
Lines 97–118 in `crates/oxide-app/src/app/handlers/active_bar/filter_controls.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [filter_controls](/crates/oxide-app/src/app/handlers/active_bar/filter_controls.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
