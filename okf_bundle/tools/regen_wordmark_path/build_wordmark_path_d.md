---
okf_version: "0.2"
type: Function
title: build_wordmark_path_d
description: "Trace 'oxide' through Panton-Bold at FONT_SIZE and return SVG path 'd'."
resource: tools/regen_wordmark_path.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:tools"
  - "domain:regen_wordmark_path.py"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:00Z"
concept_id: tools/regen_wordmark_path/build_wordmark_path_d
language: python
---

# build_wordmark_path_d

Trace 'oxide' through Panton-Bold at FONT_SIZE and return SVG path 'd'.

## Signature

```python
def build_wordmark_path_d(font: TTFont) -> str
```

## Docstring

Trace 'oxide' through Panton-Bold at FONT_SIZE and return SVG path 'd'.

## Parameters

| Name | Type | Default |
|------|------|---------|
| `font` | `TTFont` | `—` |

## Returns
`str`

## Source
Lines 41–72 in `tools/regen_wordmark_path.py`

## Relationships

| Type | Target |
|------|--------|
| related | [regen_wordmark_path](/tools/regen_wordmark_path.md) |
| called_by | [main](/tools/regen_wordmark_path/main.md) |
