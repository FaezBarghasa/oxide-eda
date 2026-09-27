---
okf_version: "0.2"
type: Function
title: toggle
description: "Toggle the checkbox for one entry, addressed by `symbol_uuid`."
resource: crates/oxide-app/src/library/updates_dialog.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/updates_dialog/toggle_1
language: rust
---

# toggle

Toggle the checkbox for one entry, addressed by `symbol_uuid`.

## Signature

```rust
pub fn toggle(&mut self, symbol_uuid: Uuid)
```

## Visibility

- `pub`

## Docstring

Toggle the checkbox for one entry, addressed by `symbol_uuid`.
No-op when the uuid isn't present (e.g. the user dismissed and
re-scanned).

## Source
Lines 182–190 in `crates/oxide-app/src/library/updates_dialog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates_dialog](/crates/oxide-app/src/library/updates_dialog.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
