---
okf_version: "0.2"
type: Function
title: sym_editor_select_part
description: "SCH Library panel: switch the editor's `active_part` to `part`."
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_select_part_1
language: rust
---

# sym_editor_select_part

SCH Library panel: switch the editor's `active_part` to `part`.

## Signature

```rust
pub(super) fn sym_editor_select_part(&mut self, part: u8) -> bool
```

## Visibility

- `pub(super)`

## Docstring

SCH Library panel: switch the editor's `active_part` to `part`.
`0` is the special Part Zero (shared pins). Clamps `part` to
`[0, max_part]` so a stale tree click can't park the editor
outside the symbol's actual range.

## Source
Lines 286–299 in `crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol.md) |
| calls | [max_part_number](/crates/oxide-app/src/library/editor/symbol/state/mod/max_part_number.md) |
