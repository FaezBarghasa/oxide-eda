---
okf_version: "0.2"
type: Function
title: set_alternate_status
description: "Set alternate `idx`'s lifecycle status."
resource: crates/oxide-app/src/library/component_preview/updates/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/supply/set_alternate_status
language: rust
---

# set_alternate_status

Set alternate `idx`'s lifecycle status.

## Signature

```rust
pub(super) fn set_alternate_status(
    state: &mut ComponentPreviewState,
    idx: usize,
    value: oxide_library::AlternateStatus,
)
```

## Visibility

- `pub(super)`

## Docstring

Set alternate `idx`'s lifecycle status.

## Source
Lines 72–81 in `crates/oxide-app/src/library/component_preview/updates/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/component_preview/updates/supply.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
