---
okf_version: "0.2"
type: Function
title: commit_number
description: "Commit a numeric parameter from its edit buffer, ignoring a buffer"
resource: crates/oxide-app/src/library/component_preview/updates/parameters.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/parameters/commit_number
language: rust
---

# commit_number

Commit a numeric parameter from its edit buffer, ignoring a buffer

## Signature

```rust
pub(super) fn commit_number(state: &mut ComponentPreviewState, name: String)
```

## Visibility

- `pub(super)`

## Docstring

Commit a numeric parameter from its edit buffer, ignoring a buffer
that does not parse as `f64`.

## Source
Lines 29–39 in `crates/oxide-app/src/library/component_preview/updates/parameters.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameters](/crates/oxide-app/src/library/component_preview/updates/parameters.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
