use dotagent::{parse, validate, Severity};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args[0] == "-h" || args[0] == "--help" {
        println!("usage: dotagent <command> <file.agent>");
        println!();
        println!("commands:");
        println!("  validate   Parse and validate an .agent file");
        println!("  parse      Parse and print an .agent file");
        process::exit(0);
    }

    let command = &args[0];
    if args.len() < 2 {
        eprintln!("error: {command} requires a file argument");
        process::exit(1);
    }

    let path = &args[1];
    let code = match command.as_str() {
        "validate" => cmd_validate(path),
        "parse" => cmd_parse(path),
        _ => {
            eprintln!("error: unknown command '{command}'");
            1
        }
    };
    process::exit(code);
}

fn cmd_validate(path: &str) -> i32 {
    let config = match parse(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("parse error: {e}");
            return 1;
        }
    };

    let errors = validate(&config);
    if errors.is_empty() {
        println!("{path}: valid");
        return 0;
    }

    for err in &errors {
        println!("  {err}");
    }

    let error_count = errors.iter().filter(|e| e.severity == Severity::Error).count();
    let warn_count = errors.iter().filter(|e| e.severity == Severity::Warning).count();

    let mut parts = Vec::new();
    if error_count > 0 {
        parts.push(format!("{error_count} error(s)"));
    }
    if warn_count > 0 {
        parts.push(format!("{warn_count} warning(s)"));
    }
    println!("{path}: {}", parts.join(", "));

    if error_count > 0 { 1 } else { 0 }
}

fn cmd_parse(path: &str) -> i32 {
    let config = match parse(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("parse error: {e}");
            return 1;
        }
    };

    if let Some(name) = &config.name {
        println!("name: {name}");
    }
    if let Some(extends) = &config.extends {
        println!("extends: {extends}");
    }
    if let Some(mode) = &config.mode {
        println!("mode: {mode}");
    }
    if let Some(desc) = &config.description {
        println!("description: {desc}");
    }
    if config.name.is_some() || config.extends.is_some() || config.mode.is_some() {
        println!();
    }

    for (name, section) in &config.sections {
        println!("[{name}]");
        for (k, v) in &section.entries {
            if k == "_items" {
                if let dotagent::Value::List(items) = v {
                    for item in items {
                        println!("  - {item}");
                    }
                }
            } else {
                println!("  {k} = {v}");
            }
        }
        println!();
    }

    0
}
