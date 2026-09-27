---
okf_version: "0.2"
type: Function
title: fuzzy_score
description: "Sublime-text-style fuzzy score. Returns `None` if any query"
resource: crates/oxide-app/src/app/command_palette.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/command_palette/fuzzy_score
language: rust
---

# fuzzy_score

Sublime-text-style fuzzy score. Returns `None` if any query

## Signature

```rust
pub fn fuzzy_score(query: &str, target: &str) -> Option<i32>
```

## Visibility

- `pub`

## Docstring

Sublime-text-style fuzzy score. Returns `None` if any query
character is missing from `target`. Higher is better. Bonuses:
word-boundary match (+10), contiguous match (+15), full-substring
match (+25). Penalty: target length (-len/4) so shorter labels
rank above longer ones with the same match quality. The contiguous
bonus dominates the word-boundary bonus so "save" → "Save File"
outranks "save" → "Set Animation Variant Edit" (acronym matches
rank below literal word matches, which matches VS Code's feel).

## Source
Lines 267–331 in `crates/oxide-app/src/app/command_palette.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_palette](/crates/oxide-app/src/app/command_palette.md) |
| called_by | [contiguous_outscores_split](/crates/oxide-app/src/app/command_palette/contiguous_outscores_split.md) |
| called_by | [prefix_outscores_substring](/crates/oxide-app/src/app/command_palette/prefix_outscores_substring.md) |
| called_by | [rank_results](/crates/oxide-app/src/app/command_palette/rank_results.md) |
| called_by | [word_boundary_bonus](/crates/oxide-app/src/app/command_palette/word_boundary_bonus.md) |
