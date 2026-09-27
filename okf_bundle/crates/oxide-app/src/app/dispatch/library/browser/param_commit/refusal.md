---
okf_version: "0.2"
type: Function
title: refusal
resource: crates/oxide-app/src/app/dispatch/library/browser/param_commit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/param_commit/refusal
language: rust
---

# refusal

## Signature

```rust
fn refusal(expected: &'static str, existing: Option<&ParamValue>) -> ParamRefusal
```

## Source
Lines 83–88 in `crates/oxide-app/src/app/dispatch/library/browser/param_commit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [param_commit](/crates/oxide-app/src/app/dispatch/library/browser/param_commit.md) |
| called_by | [param_value_for_commit](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/param_value_for_commit.md) |
