---
okf_version: "0.2"
type: Function
title: replace_path_d
description: "Replace the 'd' attribute of <path id=\"text11\"> with new_d."
resource: tools/regen_wordmark_path.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:tools"
  - "domain:regen_wordmark_path.py"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:00Z"
concept_id: tools/regen_wordmark_path/replace_path_d
language: python
---

# replace_path_d

Replace the 'd' attribute of <path id="text11"> with new_d.

## Signature

```python
def replace_path_d(svg_text: str, new_d: str) -> str
```

## Docstring

Replace the 'd' attribute of <path id="text11"> with new_d.

## Parameters

| Name | Type | Default |
|------|------|---------|
| `svg_text` | `str` | `—` |

| `new_d` | `str` | `—` |

## Returns
`str`

## Source
Lines 75–104 in `tools/regen_wordmark_path.py`

## Relationships

| Type | Target |
|------|--------|
| related | [regen_wordmark_path](/tools/regen_wordmark_path.md) |
| called_by | [main](/tools/regen_wordmark_path/main.md) |
