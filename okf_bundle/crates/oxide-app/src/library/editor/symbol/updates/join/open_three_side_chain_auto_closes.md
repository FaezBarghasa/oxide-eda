---
okf_version: "0.2"
type: Function
title: open_three_side_chain_auto_closes
description: 3 of the 4 sides selected (open chain) auto-closes via the
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/open_three_side_chain_auto_closes
language: rust
---

# open_three_side_chain_auto_closes

3 of the 4 sides selected (open chain) auto-closes via the

## Signature

```rust
fn open_three_side_chain_auto_closes()
```

## Decorators

- `test`

## Docstring

3 of the 4 sides selected (open chain) auto-closes via the
missing edge, still produces one polygon, and surfaces the
auto-close as an informational (not error) status message —
the synthetic edge is a silent-but-consequential change to
the user's selection that's worth telling them about.
[test]

## Source
Lines 276–298 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/new_editor.md) |
| calls | [push_line](/crates/oxide-app/src/library/editor/symbol/updates/join/push_line.md) |
| calls | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
