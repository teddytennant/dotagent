# Specification

An `.agent` file is a plain text file with two parts:

1. **Headers** — top-level key-value pairs that identify the agent
2. **Sections** — bracketed groups of configuration

```
name: my-agent
extends: base
mode: autonomous
description: What this agent does

[section-name]
key = value
- list item
```

## Structure

Headers come first. Sections follow. Blank lines and comments (`#`) are ignored everywhere.

Everything is case-sensitive. Section and header names use lowercase with hyphens.

## File extension

`.agent` — always lowercase, no alternatives.
