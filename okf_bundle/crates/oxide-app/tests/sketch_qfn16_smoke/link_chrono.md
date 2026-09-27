---
okf_version: "0.2"
type: Function
title: _link_chrono
description: "Unused-import linter sanity: tie the chrono pull-in to this stub"
resource: crates/oxide-app/tests/sketch_qfn16_smoke.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/tests/sketch_qfn16_smoke/link_chrono
language: rust
---

# _link_chrono

Unused-import linter sanity: tie the chrono pull-in to this stub

## Signature

```rust
fn _link_chrono() -> chrono::DateTime<Utc>
```

## Docstring

Unused-import linter sanity: tie the chrono pull-in to this stub
so an `cargo check` doesn't strip it (Footprint::empty needs Utc::now()).

## Source
Lines 198–200 in `crates/oxide-app/tests/sketch_qfn16_smoke.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_qfn16_smoke](/crates/oxide-app/tests/sketch_qfn16_smoke.md) |
