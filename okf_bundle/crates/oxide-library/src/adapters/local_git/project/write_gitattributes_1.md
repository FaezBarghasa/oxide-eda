---
okf_version: "0.2"
type: Function
title: write_gitattributes
description: "Write the project's `.gitattributes` file with the v0.22 spec:"
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project/write_gitattributes_1
language: rust
---

# write_gitattributes

Write the project's `.gitattributes` file with the v0.22 spec:

## Signature

```rust
pub fn write_gitattributes(&self, use_lfs: bool) -> Result<(), LibraryError>
```

## Visibility

- `pub`

## Docstring

Write the project's `.gitattributes` file with the v0.22 spec:
- `text eol=lf` for every `.snx*` extension so git stays
line-ending stable across Windows / macOS / Linux
collaborators.
- `binary` for `.step` / `.wrl` / `.png` / `.pdf` so git
doesn't try to diff or merge them.
- `filter=lfs diff=lfs merge=lfs -text` for everything under
`assets/3d-models/**` when `use_lfs` is on. Skipped when
off — the user can opt in later via the migration modal.

**Destructive — overwrites any existing `.gitattributes`
without merging.** Callers (today: the Enable Version Control
modal in `app/handlers/dock/project_navigation.rs`) gate this
behind explicit user action. Manual edits between Enable VC
invocations are clobbered. Future v0.x can read+merge if
the user demands it; today's contract is "Enable VC rewrites
the file from scratch".

The file is staged + committed by the migration flow's
initial-commit step; callers don't need a separate commit.

## Source
Lines 338–366 in `crates/oxide-library/src/adapters/local_git/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-library/src/adapters/local_git/project.md) |
