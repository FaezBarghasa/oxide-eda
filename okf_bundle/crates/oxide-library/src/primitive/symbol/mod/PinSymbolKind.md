---
okf_version: "0.2"
type: Class
title: PinSymbolKind
description: "Decorative IEEE-style modifier glyph attached to a pin's symbol"
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/PinSymbolKind
language: rust
---

# PinSymbolKind

Decorative IEEE-style modifier glyph attached to a pin's symbol

## Signature

```rust
pub enum PinSymbolKind
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)`
- `non_exhaustive`

## Visibility

- `pub`

## Docstring

Decorative IEEE-style modifier glyph attached to a pin's symbol
body. Altium splits these across four placement zones (Inside,
Inside Edge, Outside Edge, Outside) so a pin can carry multiple
modifiers (e.g. dot + clock for an inverted clock input). The
enum is `#[non_exhaustive]` because Altium ships 30+ IEEE glyphs
and we add them as needed — `None` is the default for legacy /
fresh pins.
[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
[non_exhaustive]

## Source
Lines 53–85 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
