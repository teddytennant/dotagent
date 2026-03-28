# CLI

The `dotagent` command-line tool validates and inspects `.agent` files.

## validate

Parse and validate an `.agent` file against the schema:

```bash
dotagent validate config.agent
```

Exits `0` if valid, `1` if there are errors. Warnings don't cause a non-zero exit.

## parse

Parse and print the contents of an `.agent` file:

```bash
dotagent parse config.agent
```

Outputs the parsed structure in a readable format.
