---
okf_version: "0.2"
type: Function
title: template_with_missing_required_flag_visible
description: Validation flag visible in the computed match list when a
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params/template_with_missing_required_flag_visible
language: rust
---

# template_with_missing_required_flag_visible

Validation flag visible in the computed match list when a

## Signature

```rust
fn template_with_missing_required_flag_visible()
```

## Decorators

- `test`

## Docstring

Validation flag visible in the computed match list when a
required parameter is absent. This is what drives the "✗ missing"
amber tag in the view.
[test]

## Source
Lines 594–610 in `crates/oxide-app/src/library/editor/params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [params](/crates/oxide-app/src/library/editor/params.md) |
| calls | [missing_required_for_test](/crates/oxide-app/src/library/editor/params/missing_required_for_test.md) |
