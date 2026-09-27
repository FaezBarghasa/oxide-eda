---
okf_version: "0.2"
type: Function
title: commit_measurement
description: "Commit a measurement parameter (value + unit) from its edit buffer,"
resource: crates/oxide-app/src/library/component_preview/updates/parameters.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/parameters/commit_measurement
language: rust
---

# commit_measurement

Commit a measurement parameter (value + unit) from its edit buffer,

## Signature

```rust
pub(super) fn commit_measurement(state: &mut ComponentPreviewState, name: String, unit: String)
```

## Visibility

- `pub(super)`

## Docstring

Commit a measurement parameter (value + unit) from its edit buffer,
ignoring a buffer that does not parse as `f64`.

## Source
Lines 48–58 in `crates/oxide-app/src/library/component_preview/updates/parameters.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameters](/crates/oxide-app/src/library/component_preview/updates/parameters.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
