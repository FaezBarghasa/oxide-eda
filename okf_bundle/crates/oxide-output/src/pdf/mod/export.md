---
okf_version: "0.2"
type: Function
title: export
resource: crates/oxide-output/src/pdf/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-output/src/pdf/mod/export
language: rust
---

# export

## Signature

```rust
impl PdfExporter { fn export(
        &self,
        ctx: &ExportContext,
        opts: &Self::Options,
    ) -> Result<Self::Output, Self::Error> }
```

## Source
Lines 271–398 in `crates/oxide-output/src/pdf/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pdf](/crates/oxide-output/src/pdf/mod.md) |
| calls | [resolve_page_range](/crates/oxide-output/src/pdf/content/resolve_page_range.md) |
| calls | [build_expression_tables](/crates/oxide-output/src/expression/build_expression_tables.md) |
| calls | [build_bookmarks](/crates/oxide-output/src/pdf/bookmarks/mod/build_bookmarks.md) |
| calls | [build_page_content](/crates/oxide-output/src/pdf/content/build_page_content.md) |
| calls | [emit_bookmarks](/crates/oxide-output/src/pdf/bookmarks/mod/emit_bookmarks.md) |
