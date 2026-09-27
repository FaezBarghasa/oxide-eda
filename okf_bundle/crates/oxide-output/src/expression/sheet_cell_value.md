---
okf_version: "0.2"
type: Function
title: sheet_cell_value
resource: crates/oxide-output/src/expression.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T19:55:16Z"
concept_id: crates/oxide-output/src/expression/sheet_cell_value
language: rust
---

# sheet_cell_value

## Signature

```rust
pub fn sheet_cell_value(sheet: &SheetSnapshot) -> String
```

## Visibility

- `pub`

## Source
Lines 29–36 in `crates/oxide-output/src/expression.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [expression](/crates/oxide-output/src/expression.md) |
| called_by | [build_page_content](/crates/oxide-output/src/pdf/content/build_page_content.md) |
| called_by | [rasterize_page](/crates/oxide-output/src/preview/rasterize/rasterize_page.md) |
