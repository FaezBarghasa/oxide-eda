---
okf_version: "0.2"
type: Function
title: set_alternate_mpn
description: "Set alternate `idx`'s manufacturer part number."
resource: crates/oxide-app/src/library/component_preview/updates/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/supply/set_alternate_mpn
language: rust
---

# set_alternate_mpn

Set alternate `idx`'s manufacturer part number.

## Signature

```rust
pub(super) fn set_alternate_mpn(state: &mut ComponentPreviewState, idx: usize, value: String)
```

## Visibility

- `pub(super)`

## Docstring

Set alternate `idx`'s manufacturer part number.

## Source
Lines 64–69 in `crates/oxide-app/src/library/component_preview/updates/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/component_preview/updates/supply.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
