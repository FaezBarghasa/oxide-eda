---
okf_version: "0.2"
type: Function
title: open_sibling_returns_the_primitive_open_task
description: "Regression (#99 part 1): this method used to return `bool` and"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/open_sibling_returns_the_primitive_open_task
language: rust
---

# open_sibling_returns_the_primitive_open_task

Regression (#99 part 1): this method used to return `bool` and

## Signature

```rust
fn open_sibling_returns_the_primitive_open_task()
```

## Decorators

- `test`

## Docstring

Regression (#99 part 1): this method used to return `bool` and
discard `handle_open_primitive`'s `Task` via `let _ = ...`. This
is a pure compile-time tripwire — the assignment only type-checks
if the signature stays `fn(&mut Oxide, &Path) -> Task<Message>`;
it stops compiling (not merely failing) if that regresses back to
`bool`. No runtime setup needed, so no `Oxide::new()` / tempdir.
[test]

## Source
Lines 148–151 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library.md) |
