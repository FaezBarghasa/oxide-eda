---
okf_version: "0.2"
type: Function
title: a_bool_cell_refuses_a_buffer_it_cannot_read_rather_than_retyping
description: "#612 — the defect itself. An unreadable buffer used to become"
resource: crates/oxide-app/src/app/dispatch/library/browser/param_commit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/param_commit/a_bool_cell_refuses_a_buffer_it_cannot_read_rather_than_retyping
language: rust
---

# a_bool_cell_refuses_a_buffer_it_cannot_read_rather_than_retyping

#612 — the defect itself. An unreadable buffer used to become

## Signature

```rust
fn a_bool_cell_refuses_a_buffer_it_cannot_read_rather_than_retyping()
```

## Decorators

- `test`

## Docstring

#612 — the defect itself. An unreadable buffer used to become
`Text`, taking the boolean type with it and committing that to
the library, so anything keying on the type lost the row.
[test]

## Source
Lines 175–186 in `crates/oxide-app/src/app/dispatch/library/browser/param_commit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [param_commit](/crates/oxide-app/src/app/dispatch/library/browser/param_commit.md) |
| calls | [param_value_for_commit](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/param_value_for_commit.md) |
