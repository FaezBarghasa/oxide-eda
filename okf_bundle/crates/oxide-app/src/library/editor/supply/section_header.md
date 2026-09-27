---
okf_version: "0.2"
type: Function
title: section_header
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply/section_header
language: rust
---

# section_header

## Signature

```rust
fn section_header(label: &'a str, tokens: &'a ThemeTokens) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 146–149 in `crates/oxide-app/src/library/editor/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/editor/supply.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
