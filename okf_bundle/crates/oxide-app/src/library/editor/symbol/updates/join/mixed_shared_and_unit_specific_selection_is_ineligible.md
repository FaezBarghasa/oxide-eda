---
okf_version: "0.2"
type: Function
title: mixed_shared_and_unit_specific_selection_is_ineligible
description: A selection mixing a shared (part 0) source with an
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/mixed_shared_and_unit_specific_selection_is_ineligible
language: rust
---

# mixed_shared_and_unit_specific_selection_is_ineligible

A selection mixing a shared (part 0) source with an

## Signature

```rust
fn mixed_shared_and_unit_specific_selection_is_ineligible()
```

## Decorators

- `test`

## Docstring

A selection mixing a shared (part 0) source with an
active-unit source is disqualified outright: no mutation, no
undo entry, and a status message explaining why — distinct
from the silent no-op every other ineligibility reason gets.
[test]

## Source
Lines 481–506 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/new_editor.md) |
| calls | [push_line](/crates/oxide-app/src/library/editor/symbol/updates/join/push_line.md) |
| calls | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
