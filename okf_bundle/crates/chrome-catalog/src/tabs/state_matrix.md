---
okf_version: "0.2"
type: Function
title: state_matrix
resource: crates/chrome-catalog/src/tabs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:chrome-catalog"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/chrome-catalog/src/tabs/state_matrix
language: rust
---

# state_matrix

## Signature

```rust
pub(crate) fn state_matrix(tokens: &ThemeTokens) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Visibility

- `pub(crate)`

## Source
Lines 42–62 in `crates/chrome-catalog/src/tabs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tabs](/crates/chrome-catalog/src/tabs.md) |
| calls | [tab](/crates/chrome-catalog/src/tabs/tab.md) |
| calls | [strip_with_baseline](/crates/chrome-catalog/src/tabs/strip_with_baseline.md) |
