# dotagent

A file format for configuring autonomous AI agents.

`.agent` files replace freeform markdown with a structured, parseable format that signals intent: **this file configures an agent.**

## Why not markdown?

Markdown is a document format for humans. When you write a `CLAUDE.md` or a memory file, you're shoehorning agent instructions into a format designed for rendering prose. The file extension tells every tool "this is a document." Nothing in the ecosystem knows it's a behavioral contract for an AI agent.

## What .agent gives you

- **Intent.** The extension tells tools, editors, and humans what this file is for.
- **Structure.** Defined headers and sections that agents parse deterministically.
- **Validation.** A schema you can check before an agent runs.
- **Composability.** Inheritance via `extends` for shared base configurations.
- **Discoverability.** `*.agent` is greppable, globbable, unambiguous.

## Quick look

```
name: maintainer
mode: autonomous

[constraints]
- tests must pass before commit
- never force push

[schedule]
interval = 10m
```
