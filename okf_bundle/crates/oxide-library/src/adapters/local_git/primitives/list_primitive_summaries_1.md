---
okf_version: "0.2"
type: Function
title: list_primitive_summaries
description: "One [`PrimitiveSummary`] per `<uuid>.<ext>` file directly under"
resource: crates/oxide-library/src/adapters/local_git/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/local_git/primitives/list_primitive_summaries_1
language: rust
---

# list_primitive_summaries

One [`PrimitiveSummary`] per `<uuid>.<ext>` file directly under

## Signature

```rust
pub(super) fn list_primitive_summaries(
        &self,
        kind: PrimitiveKind,
    ) -> Result<Vec<PrimitiveSummary>, LibraryError>
```

## Visibility

- `pub(super)`

## Docstring

One [`PrimitiveSummary`] per `<uuid>.<ext>` file directly under
`<root>/<subdir>`, sorted by name.

Only the primitive's *name* survives, so the envelope is parsed
into its concrete type and `.name` lifted straight off it. This
used to be generic over `T: DeserializeOwned` with a `name_of`
extractor, which forced a full `serde_json` serialise +
deserialise **per file** purely to teach the compiler that
`T == Footprint` / `T == SimModel` — every byte of which was
discarded on the next line (#99).

## Source
Lines 436–476 in `crates/oxide-library/src/adapters/local_git/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library/src/adapters/local_git/primitives.md) |
| calls | [primitive_name_in_envelope](/crates/oxide-library/src/adapters/local_git/primitives/primitive_name_in_envelope.md) |
