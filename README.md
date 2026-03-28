# dotagent

A file format for configuring autonomous AI agents.

`.agent` files replace freeform markdown with a structured, parseable format that signals intent: *this file configures an agent.*

## Install

```bash
cargo install dotagent
```

## Usage

```bash
# validate an agent config
dotagent validate config.agent

# parse and print
dotagent parse config.agent
```

## Format

```
name: maintainer
extends: base

[identity]
author = "Jane Doe"

[constraints]
- tests must pass before commit
- never force push

[scope]
paths = ["/home/user/repos/**"]
```

See the [full specification](https://dotagent.dev) for details.

## License

MIT
