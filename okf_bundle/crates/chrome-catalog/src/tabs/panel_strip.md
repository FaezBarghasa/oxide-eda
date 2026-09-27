---
okf_version: "0.2"
type: Function
title: panel_strip
resource: crates/chrome-catalog/src/tabs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:chrome-catalog"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/chrome-catalog/src/tabs/panel_strip
language: rust
---

# panel_strip

## Signature

```rust
pub(crate) fn panel_strip(tokens: &ThemeTokens) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Visibility

- `pub(crate)`

## Source
Lines 64–87 in `crates/chrome-catalog/src/tabs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tabs](/crates/chrome-catalog/src/tabs.md) |
| calls | [tab](/crates/chrome-catalog/src/tabs/tab.md) |
| calls | [strip_with_baseline](/crates/chrome-catalog/src/tabs/strip_with_baseline.md) |
