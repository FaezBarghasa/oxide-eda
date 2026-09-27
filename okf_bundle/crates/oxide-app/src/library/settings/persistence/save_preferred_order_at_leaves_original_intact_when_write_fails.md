---
okf_version: "0.2"
type: Function
title: save_preferred_order_at_leaves_original_intact_when_write_fails
description: "`save_preferred_order_at` must go through `atomic_write`, not"
resource: crates/oxide-app/src/library/settings/persistence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/settings/persistence/save_preferred_order_at_leaves_original_intact_when_write_fails
language: rust
---

# save_preferred_order_at_leaves_original_intact_when_write_fails

`save_preferred_order_at` must go through `atomic_write`, not

## Signature

```rust
fn save_preferred_order_at_leaves_original_intact_when_write_fails()
```

## Decorators

- `test`

## Docstring

`save_preferred_order_at` must go through `atomic_write`, not
`fs::write`: a failed save leaves the previously persisted order
fully intact, and reports the failure instead of swallowing it.

Discriminator: denying new-file creation in the destination's
parent directory makes `atomic_write`'s `File::create(&tmp)` fail
before it can touch the destination, regardless of the unique
per-writer temp name it picks (#416). A plain `fs::write` would
ignore that and clobber the old file — so this test fails on a
revert.
[test]

## Source
Lines 239–252 in `crates/oxide-app/src/library/settings/persistence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persistence](/crates/oxide-app/src/library/settings/persistence.md) |
| calls | [config_path_for_dir](/crates/oxide-app/src/library/settings/persistence/config_path_for_dir.md) |
| calls | [save_preferred_order_at](/crates/oxide-app/src/library/settings/persistence/save_preferred_order_at.md) |
