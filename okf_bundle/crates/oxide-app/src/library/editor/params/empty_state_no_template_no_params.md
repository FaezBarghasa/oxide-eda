---
okf_version: "0.2"
type: Function
title: empty_state_no_template_no_params
description: "Empty state path: no template registered for the component class"
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params/empty_state_no_template_no_params
language: rust
---

# empty_state_no_template_no_params

Empty state path: no template registered for the component class

## Signature

```rust
fn empty_state_no_template_no_params()
```

## Decorators

- `test`

## Docstring

Empty state path: no template registered for the component class
AND no parameters bound. The validation list must be empty (a
missing template is "trivially passes" per the registry contract)
and `resolve` returns `None`.
[test]

## Source
Lines 544–554 in `crates/oxide-app/src/library/editor/params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [params](/crates/oxide-app/src/library/editor/params.md) |
