# Headers

Headers are top-level key-value pairs before any section. They identify the agent.

```
name: maintainer
extends: base
mode: autonomous
description: Cycles through repos and improves them
```

## Required

| Header | Description |
|--------|-------------|
| `name` | Unique identifier for this agent |

## Optional

| Header | Description |
|--------|-------------|
| `extends` | Name of a parent .agent file to inherit from |
| `mode` | One of: `autonomous`, `interactive`, `supervised`, `passive` |
| `description` | Human-readable summary |

## Modes

| Mode | Meaning |
|------|---------|
| `autonomous` | Runs without human input |
| `interactive` | Works with a human in the loop |
| `supervised` | Runs autonomously but requires approval for actions |
| `passive` | Read-only, observation only |
