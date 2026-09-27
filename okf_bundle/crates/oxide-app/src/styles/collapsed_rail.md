---
okf_version: "0.2"
type: Function
title: collapsed_rail
description: Collapsed rail (vertical/horizontal panel strip)
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/collapsed_rail
language: rust
---

# collapsed_rail

Collapsed rail (vertical/horizontal panel strip)

## Signature

```rust
pub fn collapsed_rail(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Collapsed rail (vertical/horizontal panel strip)

## Source
Lines 158–170 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_rail](/crates/oxide-app/src/dock/view/view_rail.md) |
