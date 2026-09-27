---
okf_version: "0.2"
type: Function
title: unresolved_refs
description: "Sweep a stream of references, splitting the ones that genuinely"
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/unresolved_refs_1
language: rust
---

# unresolved_refs

Sweep a stream of references, splitting the ones that genuinely

## Signature

```rust
pub fn unresolved_refs(&self, refs: I) -> UnresolvedRefs
```

## Type Parameters

- `'a`
- `I`

## Visibility

- `pub`

## Docstring

Sweep a stream of references, splitting the ones that genuinely
don't resolve from the ones whose lookup failed. See
[`UnresolvedRefs`] — the two lists carry different meanings and
must not be shown to the user with the same wording.

## Source
Lines 220–237 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
| calls | [probe_every_kind](/crates/oxide-library/src/adapters/library_set/probe_every_kind.md) |
