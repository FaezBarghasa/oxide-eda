---
okf_version: "0.2"
type: Function
title: report_read_failure
description: "Flatten a resolution into the `Option` the UI holds, sending any"
resource: crates/oxide-app/src/library/resolve.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/resolve/report_read_failure
language: rust
---

# report_read_failure

Flatten a resolution into the `Option` the UI holds, sending any

## Signature

```rust
pub(crate) fn report_read_failure(
    resolved: Result<Option<T>, LibraryError>,
    kind: ResolvedKind,
    reference: &PrimitiveRef,
    context: &str,
) -> Option<T>
```

## Type Parameters

- `T`

## Visibility

- `pub(crate)`

## Docstring

Flatten a resolution into the `Option` the UI holds, sending any
read failure to the Messages panel on the way past.

`context` names the gesture that triggered the lookup so the record
says which action came up empty.

## Source
Lines 42–67 in `crates/oxide-app/src/library/resolve.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resolve](/crates/oxide-app/src/library/resolve.md) |
| called_by | [handle_select_preview_tab](/crates/oxide-app/src/app/dispatch/library/component_preview/handle_select_preview_tab.md) |
| called_by | [apply_primitive_pick_to_preview](/crates/oxide-app/src/app/dispatch/library/primitive_picker/apply_primitive_pick_to_preview.md) |
| called_by | [a_resolved_primitive_passes_straight_through](/crates/oxide-app/src/library/resolve/a_resolved_primitive_passes_straight_through.md) |
