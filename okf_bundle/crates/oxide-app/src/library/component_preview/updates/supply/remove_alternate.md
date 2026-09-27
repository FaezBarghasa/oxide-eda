---
okf_version: "0.2"
type: Function
title: remove_alternate
description: "Remove alternate `idx` when it is in range."
resource: crates/oxide-app/src/library/component_preview/updates/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/supply/remove_alternate
language: rust
---

# remove_alternate

Remove alternate `idx` when it is in range.

## Signature

```rust
pub(super) fn remove_alternate(state: &mut ComponentPreviewState, idx: usize)
```

## Visibility

- `pub(super)`

## Docstring

Remove alternate `idx` when it is in range.

## Source
Lines 96–101 in `crates/oxide-app/src/library/component_preview/updates/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/component_preview/updates/supply.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
