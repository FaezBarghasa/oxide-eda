---
okf_version: "0.2"
type: Function
title: discarding_puts_all_three_symbol_drafts_back
description: "The bug as the user met it: change the Symbol Editor grid style, press"
resource: crates/oxide-app/tests/regression/preferences_symbol_drafts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_symbol_drafts/discarding_puts_all_three_symbol_drafts_back
language: rust
---

# discarding_puts_all_three_symbol_drafts_back

The bug as the user met it: change the Symbol Editor grid style, press

## Signature

```rust
fn discarding_puts_all_three_symbol_drafts_back()
```

## Decorators

- `test`

## Docstring

The bug as the user met it: change the Symbol Editor grid style, press
Cancel, and the change survived. `revert_preferences_drafts` restored
the four schematic appearance settings and skipped these three.
[test]

## Source
Lines 81–125 in `crates/oxide-app/tests/regression/preferences_symbol_drafts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_symbol_drafts](/crates/oxide-app/tests/regression/preferences_symbol_drafts.md) |
| calls | [inner](/crates/oxide-app/tests/regression/preferences_symbol_drafts/inner.md) |
| calls | [other_grid_style](/crates/oxide-app/tests/regression/preferences_symbol_drafts/other_grid_style.md) |
| calls | [other_pin_selection](/crates/oxide-app/tests/regression/preferences_symbol_drafts/other_pin_selection.md) |
