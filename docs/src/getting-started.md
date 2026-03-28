# Getting Started

## Install

```bash
cargo install dotagent
```

## Create your first .agent file

Create a file called `assistant.agent`:

```
name: assistant
mode: interactive

[constraints]
- prefer minimal changes
- explain before modifying
```

## Validate it

```bash
dotagent validate assistant.agent
```

## Use it as a library

```rust
use dotagent::{parse, validate};

fn main() {
    let config = parse("assistant.agent").unwrap();
    println!("{}", config.name.unwrap());

    let errors = validate(&config);
    assert!(errors.is_empty());
}
```
