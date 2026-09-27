---
okf_version: "0.2"
type: Function
title: apply_upload_result
description: "Apply the result of a pinned-PDF upload: hash the bytes with SHA-256"
resource: crates/oxide-app/src/library/component_preview/updates/datasheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:28:52Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/datasheet/apply_upload_result
language: rust
---

# apply_upload_result

Apply the result of a pinned-PDF upload: hash the bytes with SHA-256

## Signature

```rust
pub(super) fn apply_upload_result(
    state: &mut ComponentPreviewState,
    payload: Option<(Vec<u8>, String)>,
)
```

## Visibility

- `pub(super)`

## Docstring

Apply the result of a pinned-PDF upload: hash the bytes with SHA-256
and pin the datasheet to that content hash. A cancelled pick (`None`)
leaves the row untouched.

## Source
Lines 52–65 in `crates/oxide-app/src/library/component_preview/updates/datasheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [datasheet](/crates/oxide-app/src/library/component_preview/updates/datasheet.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
