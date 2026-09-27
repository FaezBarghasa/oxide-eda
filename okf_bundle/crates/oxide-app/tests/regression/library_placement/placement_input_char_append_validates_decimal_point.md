---
okf_version: "0.2"
type: Function
title: placement_input_char_append_validates_decimal_point
description: "v0.24 Track D — typed character path. The user types '5' then"
resource: crates/oxide-app/tests/regression/library_placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_placement/placement_input_char_append_validates_decimal_point
language: rust
---

# placement_input_char_append_validates_decimal_point

v0.24 Track D — typed character path. The user types '5' then

## Signature

```rust
fn placement_input_char_append_validates_decimal_point()
```

## Decorators

- `test`

## Docstring

v0.24 Track D — typed character path. The user types '5' then
'.', then '2' against an active Line tool with first click
landed; the dispatcher's char-append handler must validate
(single decimal point) and grow `buffer = "5.2"` keyed off
`LineLength`.
[test]

## Source
Lines 439–496 in `crates/oxide-app/tests/regression/library_placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_placement](/crates/oxide-app/tests/regression/library_placement.md) |
