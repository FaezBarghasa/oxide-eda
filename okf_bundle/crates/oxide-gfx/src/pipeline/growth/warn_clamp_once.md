---
okf_version: "0.2"
type: Function
title: warn_clamp_once
resource: crates/oxide-gfx/src/pipeline/growth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/pipeline/growth/warn_clamp_once
language: rust
---

# warn_clamp_once

## Signature

```rust
fn warn_clamp_once(label: &'static str, required: usize, writable: usize)
```

## Source
Lines 71–79 in `crates/oxide-gfx/src/pipeline/growth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [growth](/crates/oxide-gfx/src/pipeline/growth.md) |
| called_by | [ensure_capacity](/crates/oxide-gfx/src/pipeline/growth/ensure_capacity.md) |
