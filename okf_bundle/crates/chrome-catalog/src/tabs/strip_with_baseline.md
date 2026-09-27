---
okf_version: "0.2"
type: Function
title: strip_with_baseline
resource: crates/chrome-catalog/src/tabs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:chrome-catalog"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/chrome-catalog/src/tabs/strip_with_baseline
language: rust
---

# strip_with_baseline

## Signature

```rust
fn strip_with_baseline(
    tabs: Row<'a, Message>,
    accent_position: AccentPosition,
    tokens: &ThemeTokens,
) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Source
Lines 89–129 in `crates/chrome-catalog/src/tabs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tabs](/crates/chrome-catalog/src/tabs.md) |
| calls | [color](/crates/chrome-catalog/src/theme/color.md) |
| called_by | [document_strip](/crates/chrome-catalog/src/tabs/document_strip.md) |
| called_by | [panel_strip](/crates/chrome-catalog/src/tabs/panel_strip.md) |
| called_by | [state_matrix](/crates/chrome-catalog/src/tabs/state_matrix.md) |
