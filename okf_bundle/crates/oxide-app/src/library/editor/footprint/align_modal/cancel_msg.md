---
okf_version: "0.2"
type: Function
title: cancel_msg
description: "The `AlignCancel` message for the editor at `path` — reused by the"
resource: crates/oxide-app/src/library/editor/footprint/align_modal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/align_modal/cancel_msg
language: rust
---

# cancel_msg

The `AlignCancel` message for the editor at `path` — reused by the

## Signature

```rust
fn cancel_msg(path: &Path) -> LibraryMessage
```

## Docstring

The `AlignCancel` message for the editor at `path` — reused by the
header ✕ and the footer Cancel button.

## Source
Lines 177–182 in `crates/oxide-app/src/library/editor/footprint/align_modal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [align_modal](/crates/oxide-app/src/library/editor/footprint/align_modal.md) |
