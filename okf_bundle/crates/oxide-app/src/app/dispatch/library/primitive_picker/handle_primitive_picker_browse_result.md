---
okf_version: "0.2"
type: Function
title: handle_primitive_picker_browse_result
description: Filesystem-picked primitive — auto-mount the containing
resource: crates/oxide-app/src/app/dispatch/library/primitive_picker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/primitive_picker/handle_primitive_picker_browse_result
language: rust
---

# handle_primitive_picker_browse_result

Filesystem-picked primitive — auto-mount the containing

## Signature

```rust
impl Oxide { fn handle_primitive_picker_browse_result(&mut self, file: std::path::PathBuf) -> Task<Message> }
```

## Docstring

Filesystem-picked primitive — auto-mount the containing
`.snxlib`, then synthesize a Pick.

## Source
Lines 313–376 in `crates/oxide-app/src/app/dispatch/library/primitive_picker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitive_picker](/crates/oxide-app/src/app/dispatch/library/primitive_picker.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
