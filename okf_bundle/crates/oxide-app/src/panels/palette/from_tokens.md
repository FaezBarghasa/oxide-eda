---
okf_version: "0.2"
type: Function
title: from_tokens
description: Derive the panel palette from the active theme tokens.
resource: crates/oxide-app/src/panels/palette.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/palette/from_tokens
language: rust
---

# from_tokens

Derive the panel palette from the active theme tokens.

## Signature

```rust
impl PanelPalette { pub fn from_tokens(tokens: &ThemeTokens) -> Self }
```

## Visibility

- `pub`

## Docstring

Derive the panel palette from the active theme tokens.

Reproduces, verbatim, the per-call-site derivations this struct
replaced — same helpers, same token fields, same brighten maths.

## Source
Lines 55–72 in `crates/oxide-app/src/panels/palette.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [palette](/crates/oxide-app/src/panels/palette.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
