---
okf_version: "0.2"
type: Function
title: bounds_grows_toward_positive_y_for_an_up_pin
description: "Regression test for the `bounds()` half of #495: the pin-tip"
resource: crates/oxide-widgets/src/symbol_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/symbol_preview/bounds_grows_toward_positive_y_for_an_up_pin
language: rust
---

# bounds_grows_toward_positive_y_for_an_up_pin

Regression test for the `bounds()` half of #495: the pin-tip

## Signature

```rust
fn bounds_grows_toward_positive_y_for_an_up_pin()
```

## Decorators

- `test`

## Docstring

Regression test for the `bounds()` half of #495: the pin-tip
expansion must stay in the same library-space (Y-up) convention as
the rest of `bounds()`, so an Up pin grows the box toward +y, not
-y (the box's vertical midpoint tells them apart).
[test]

## Source
Lines 455–463 in `crates/oxide-widgets/src/symbol_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_preview](/crates/oxide-widgets/src/symbol_preview.md) |
| calls | [symbol_with_pin](/crates/oxide-widgets/src/symbol_preview/symbol_with_pin.md) |
