---
okf_version: "0.2"
type: Function
title: resolve_template
description: Resolve the parameter template for a preview state by walking
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params/resolve_template
language: rust
---

# resolve_template

Resolve the parameter template for a preview state by walking

## Signature

```rust
fn resolve_template(
    state: &ComponentPreviewState,
    library_state: &'a LibraryState,
) -> Option<&'a ParameterTemplate>
```

## Type Parameters

- `'a`

## Docstring

Resolve the parameter template for a preview state by walking
`LibraryState.open_libraries` for the matching `library_path`, then
asking the registry for the entry under `(library_id, class)`.

## Source
Lines 33–47 in `crates/oxide-app/src/library/editor/params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [params](/crates/oxide-app/src/library/editor/params.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/params/view.md) |
