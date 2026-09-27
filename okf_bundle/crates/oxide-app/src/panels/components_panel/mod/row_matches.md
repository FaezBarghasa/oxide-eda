---
okf_version: "0.2"
type: Function
title: row_matches
description: Substring filter — case-insensitive match on mpn / manufacturer /
resource: crates/oxide-app/src/panels/components_panel/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/mod/row_matches
language: rust
---

# row_matches

Substring filter — case-insensitive match on mpn / manufacturer /

## Signature

```rust
fn row_matches(row: &ComponentRow, library_name: &str, needle: &str) -> bool
```

## Docstring

Substring filter — case-insensitive match on mpn / manufacturer /
internal_pn / library name. Empty needle = match everything.
Stage 9 deliberately keeps this trivial; the rich `mpn:` /
`lifecycle:` syntax is a follow-up.

## Source
Lines 300–309 in `crates/oxide-app/src/panels/components_panel/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [components_panel](/crates/oxide-app/src/panels/components_panel/mod.md) |
| called_by | [view_library_block](/crates/oxide-app/src/panels/components_panel/mod/view_library_block.md) |
