---
okf_version: "0.2"
type: Function
title: format_component_title
resource: crates/oxide-output/src/pdf/bookmarks/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/bookmarks/mod/format_component_title
language: rust
---

# format_component_title

## Signature

```rust
fn format_component_title(reference: &str, value: &str, opts: &PdfOptions) -> String
```

## Source
Lines 395–401 in `crates/oxide-output/src/pdf/bookmarks/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bookmarks](/crates/oxide-output/src/pdf/bookmarks/mod.md) |
| called_by | [build_bookmarks](/crates/oxide-output/src/pdf/bookmarks/mod/build_bookmarks.md) |
