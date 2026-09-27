---
okf_version: "0.2"
type: Function
title: slot_input
description: Build the kind-appropriate editor widget for one parameter cell.
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params/slot_input
language: rust
---

# slot_input

Build the kind-appropriate editor widget for one parameter cell.

## Signature

```rust
fn slot_input(
    state: &'a ComponentPreviewState,
    name: &str,
    kind: ParamKind,
    unit: Option<String>,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

Build the kind-appropriate editor widget for one parameter cell.

## Source
Lines 282–418 in `crates/oxide-app/src/library/editor/params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [params](/crates/oxide-app/src/library/editor/params.md) |
| calls | [display_param](/crates/oxide-app/src/library/editor/params/display_param.md) |
| called_by | [custom_row](/crates/oxide-app/src/library/editor/params/custom_row.md) |
| called_by | [template_row](/crates/oxide-app/src/library/editor/params/template_row.md) |
