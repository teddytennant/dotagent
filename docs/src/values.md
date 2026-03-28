# Values

Values appear on the right side of `key = value` pairs within sections.

## Types

| Type | Syntax | Example |
|------|--------|---------|
| String | Bare or quoted | `author = Alice` or `author = "Alice Bob"` |
| Boolean | `true`/`false`/`yes`/`no` | `read-on-start = true` |
| Array | JSON syntax | `paths = ["/a", "/b"]` |
| List | Dash-prefixed lines | `- rule one` |

## Strings

Bare strings don't need quotes unless they contain special characters. Use double quotes for strings with spaces or special characters.

```
role = engineer
author = "Jane Doe <jane@example.com>"
```

## Booleans

```
read-on-start = true
write-on-end = false
```

`yes` and `no` are accepted as aliases.

## Arrays

JSON array syntax for inline lists:

```
paths = ["/home/user/repos", "/tmp/scratch"]
modes = ["maintain", "build", "cross-repo"]
```

## Lists

Dash-prefixed items for readable multi-line lists, typically used in `[constraints]`:

```
[constraints]
- tests must pass
- no force push
- prefer small commits
```

## Comments

Lines starting with `#` are ignored:

```
# This is a comment
name: test
```
