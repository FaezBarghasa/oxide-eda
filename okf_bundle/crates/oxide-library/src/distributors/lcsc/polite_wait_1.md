---
okf_version: "0.2"
type: Function
title: polite_wait
description: "Wait until at least `THROTTLE_INTERVAL` has elapsed since the last"
resource: crates/oxide-library/src/distributors/lcsc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/distributors/lcsc/polite_wait_1
language: rust
---

# polite_wait

Wait until at least `THROTTLE_INTERVAL` has elapsed since the last

## Signature

```rust
fn polite_wait(&self)
```

## Docstring

Wait until at least `THROTTLE_INTERVAL` has elapsed since the last
call. Records the current instant on exit so subsequent calls are
rate-limited consistently across threads.

## Source
Lines 79–90 in `crates/oxide-library/src/distributors/lcsc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lcsc](/crates/oxide-library/src/distributors/lcsc.md) |
