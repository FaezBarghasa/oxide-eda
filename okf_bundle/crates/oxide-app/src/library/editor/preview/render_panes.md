---
okf_version: "0.2"
type: Function
title: render_panes
resource: crates/oxide-app/src/library/editor/preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/preview/render_panes
language: rust
---

# render_panes

## Signature

```rust
fn render_panes(
    state: &'a ComponentPreviewState,
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 76–143 in `crates/oxide-app/src/library/editor/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/editor/preview.md) |
| calls | [symbol_path](/crates/oxide-app/src/library/editor/preview/symbol_path.md) |
| calls | [footprint_path](/crates/oxide-app/src/library/editor/preview/footprint_path.md) |
| calls | [render_pane](/crates/oxide-app/src/library/editor/preview/render_pane.md) |
| calls | [symbol_summary_text](/crates/oxide-app/src/library/editor/preview/symbol_summary_text.md) |
| calls | [footprint_summary_text](/crates/oxide-app/src/library/editor/preview/footprint_summary_text.md) |
