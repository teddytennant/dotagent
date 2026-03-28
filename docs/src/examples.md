# Examples

## Minimal

The smallest valid `.agent` file:

```
name: helper
mode: passive

[constraints]
- read only
```

## Interactive assistant

```
name: assistant
mode: interactive
description: A coding assistant

[identity]
author = "Jane Doe <jane@example.com>"

[constraints]
- tests must pass before commit
- never force push
- prefer minimal changes

[scope]
paths = ["/home/user/project"]
```

## Autonomous agent

A full-featured autonomous agent with scheduling and memory:

```
name: maintainer
extends: base
mode: autonomous
description: Autonomous engineering agent

[identity]
author = "Jane Doe <jane@example.com>"

[constraints]
- tests must pass before commit
- never force push
- keep changes minimal

[memory]
brain = "/home/user/agent-brain/"
read-on-start = true
write-on-end = true

[schedule]
interval = 10m
script = "/home/user/agent-loop.sh"

[scope]
paths = ["/home/user/repos/**"]
modes = ["maintain", "build", "cross-repo"]

[tools]
allowed = ["read", "write", "bash", "grep", "glob"]
```
