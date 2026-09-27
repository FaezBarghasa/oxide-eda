---
okf_version: "0.2"
type: Function
title: reload_primitive_in_library_set
description: Walk the open libraries to find one whose root contains
resource: crates/oxide-app/src/app/dispatch/library/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor/reload_primitive_in_library_set
language: rust
---

# reload_primitive_in_library_set

Walk the open libraries to find one whose root contains

## Signature

```rust
impl Oxide { fn reload_primitive_in_library_set(&mut self, _path: &std::path::Path) }
```

## Docstring

Walk the open libraries to find one whose root contains
`path` (e.g. `…/mylib.snxlib/symbols/foo.snxsym` lives under
`…/mylib.snxlib/`), and ask the matching adapter to reload
the primitive UUID encoded in the file. The adapter's
`reload_primitive` (where supported) repopulates its in-memory
cache so any Component Preview tabs that resolve through
`LibrarySet` see the new bytes on the next render.

Best-effort — returns silently when the path isn't under a
mounted library or when the adapter has no reload hook.

## Source
Lines 602–610 in `crates/oxide-app/src/app/dispatch/library/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/app/dispatch/library/editor.md) |
