---
okf_version: "0.2"
type: Function
title: open
description: "Open a `.snxsch` file from disk."
resource: crates/oxide-engine/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T12:59:13Z"
concept_id: crates/oxide-engine/src/lib/open
language: rust
---

# open

Open a `.snxsch` file from disk.

## Signature

```rust
impl Engine { pub fn open(path: &Path) -> Result<Self, EngineError> }
```

## Visibility

- `pub`

## Docstring

Open a `.snxsch` file from disk.

Foreign-format files (any extension other than `.snxsch`) are not
readable here; an optional GPL-3.0 import companion handles
conversion to `.snxsch` and is shipped separately from this
Apache-2.0 crate.

## Source
Lines 71–84 in `crates/oxide-engine/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-engine/src/lib.md) |
