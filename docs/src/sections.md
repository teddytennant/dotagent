# Sections

Sections are bracketed groups that hold related configuration.

```
[section-name]
key = value
- list item
```

## Known sections

| Section | Purpose |
|---------|---------|
| `identity` | Who the agent is — author, role |
| `constraints` | Rules the agent must follow |
| `memory` | Where and how the agent persists knowledge |
| `scope` | What files/paths/repos the agent can access |
| `schedule` | When the agent runs |
| `tools` | Which tools the agent can use |
| `environment` | Environment variables and runtime config |
| `permissions` | What the agent is allowed to do |

Unknown sections produce a validation warning but are not rejected. This keeps the format extensible.

## identity

```
[identity]
author = "Jane Doe <jane@example.com>"
role = "engineer"
```

## constraints

Constraints are typically a list of rules:

```
[constraints]
- tests must pass before commit
- never force push
- keep changes minimal
```

## memory

```
[memory]
brain = "/path/to/brain"
read-on-start = true
write-on-end = true
```

## scope

```
[scope]
paths = ["/home/user/repos/**"]
modes = ["maintain", "build"]
```

## schedule

Use either `interval` or `cron`, not both.

```
[schedule]
interval = 10m
script = "/path/to/loop.sh"
```
