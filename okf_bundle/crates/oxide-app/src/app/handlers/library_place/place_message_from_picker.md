---
okf_version: "0.2"
type: Function
title: place_message_from_picker
resource: crates/oxide-app/src/app/handlers/library_place.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/library_place/place_message_from_picker
language: rust
---

# place_message_from_picker

## Signature

```rust
pub(crate) fn place_message_from_picker(
    library_path: PathBuf,
    table: String,
    row_id: RowId,
) -> Message
```

## Decorators

- `expect(
    dead_code,
    reason = "kept next to the rest of the place-flow code so the picker handler stays small"
)`

## Visibility

- `pub(crate)`

## Source
Lines 136–146 in `crates/oxide-app/src/app/handlers/library_place.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_place](/crates/oxide-app/src/app/handlers/library_place.md) |
