---
okf_version: "0.2"
type: Function
title: list_components_filtered
description: Re-run a query against every open library — picker filter helper.
resource: crates/oxide-app/src/library/commands.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/commands/list_components_filtered
language: rust
---

# list_components_filtered

Re-run a query against every open library — picker filter helper.

## Signature

```rust
pub fn list_components_filtered(
    state: &LibraryState,
    text_filter: &str,
) -> Vec<(PathBuf, ComponentSummary)>
```

## Visibility

- `pub`

## Docstring

Re-run a query against every open library — picker filter helper.

## Source
Lines 655–676 in `crates/oxide-app/src/library/commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [commands](/crates/oxide-app/src/library/commands.md) |
| called_by | [view](/crates/oxide-app/src/library/picker/view.md) |
