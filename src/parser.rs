use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Bool(bool),
    List(Vec<String>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{s}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::List(items) => {
                let joined: Vec<_> = items.iter().map(|s| format!("\"{s}\"")).collect();
                write!(f, "[{}]", joined.join(", "))
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Section {
    pub name: String,
    pub entries: BTreeMap<String, Value>,
}

impl Section {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.get(key)
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        match self.entries.get(key) {
            Some(Value::Str(s)) => Some(s),
            _ => None,
        }
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.entries.get(key) {
            Some(Value::Bool(b)) => Some(*b),
            _ => None,
        }
    }

    pub fn get_list(&self, key: &str) -> Option<&[String]> {
        match self.entries.get(key) {
            Some(Value::List(v)) => Some(v),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AgentConfig {
    pub name: Option<String>,
    pub extends: Option<String>,
    pub mode: Option<String>,
    pub description: Option<String>,
    pub sections: BTreeMap<String, Section>,
    pub source: Option<String>,
}

impl AgentConfig {
    pub fn section(&self, name: &str) -> Option<&Section> {
        self.sections.get(name)
    }

    pub fn has_section(&self, name: &str) -> bool {
        self.sections.contains_key(name)
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: Option<usize>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.line {
            Some(n) => write!(f, "line {n}: {}", self.message),
            None => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    fn new(message: impl Into<String>, line: usize) -> Self {
        Self {
            message: message.into(),
            line: Some(line),
        }
    }
}

pub fn parse(path: impl AsRef<Path>) -> Result<AgentConfig, ParseError> {
    let path = path.as_ref();
    let text = fs::read_to_string(path).map_err(|e| ParseError {
        message: format!("cannot read {}: {e}", path.display()),
        line: None,
    })?;
    let mut config = parse_str(&text)?;
    config.source = Some(path.display().to_string());
    Ok(config)
}

pub fn parse_str(text: &str) -> Result<AgentConfig, ParseError> {
    let mut config = AgentConfig::default();
    let mut current_section: Option<String> = None;
    let mut section_entries: BTreeMap<String, Value> = BTreeMap::new();

    let flush = |config: &mut AgentConfig,
                 section_name: &Option<String>,
                 entries: &mut BTreeMap<String, Value>| {
        if let Some(name) = section_name {
            config.sections.insert(
                name.clone(),
                Section {
                    name: name.clone(),
                    entries: std::mem::take(entries),
                },
            );
        }
    };

    for (i, raw_line) in text.lines().enumerate() {
        let line_num = i + 1;
        let line = raw_line.trim();

        // Skip blank lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Section header: [name]
        if line.starts_with('[') && line.ends_with(']') {
            let name = &line[1..line.len() - 1];
            if !is_valid_ident(name) {
                return Err(ParseError::new(
                    format!("invalid section name: '{name}'"),
                    line_num,
                ));
            }
            flush(&mut config, &current_section, &mut section_entries);
            current_section = Some(name.to_string());
            continue;
        }

        // Top-level headers (before any section)
        if current_section.is_none() {
            if let Some((key, value)) = parse_header(line) {
                match key {
                    "name" => config.name = Some(value.to_string()),
                    "extends" => config.extends = Some(value.to_string()),
                    "mode" => config.mode = Some(value.to_string()),
                    "description" => config.description = Some(value.to_string()),
                    _ => {
                        return Err(ParseError::new(
                            format!("unknown top-level header: '{key}'"),
                            line_num,
                        ))
                    }
                }
                continue;
            }
            return Err(ParseError::new(
                format!("unexpected content outside section: '{line}'"),
                line_num,
            ));
        }

        // Inside a section: key = value
        if let Some((key, raw_value)) = parse_kv(line) {
            let value = parse_value(raw_value, line_num)?;
            section_entries.insert(key.to_string(), value);
            continue;
        }

        // Inside a section: - list item
        if let Some(item) = parse_list_item(line) {
            let key = "_items".to_string();
            match section_entries.get_mut(&key) {
                Some(Value::List(list)) => list.push(item.to_string()),
                _ => {
                    section_entries.insert(key, Value::List(vec![item.to_string()]));
                }
            }
            continue;
        }

        return Err(ParseError::new(
            format!("unexpected syntax: '{line}'"),
            line_num,
        ));
    }

    flush(&mut config, &current_section, &mut section_entries);
    Ok(config)
}

fn is_valid_ident(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

fn parse_header(line: &str) -> Option<(&str, &str)> {
    let colon = line.find(':')?;
    let key = line[..colon].trim();
    if !is_valid_ident(key) {
        return None;
    }
    let value = line[colon + 1..].trim();
    Some((key, value))
}

fn parse_kv(line: &str) -> Option<(&str, &str)> {
    let eq = line.find('=')?;
    let key = line[..eq].trim();
    if !is_valid_ident(key) {
        return None;
    }
    let value = line[eq + 1..].trim();
    Some((key, value))
}

fn parse_list_item(line: &str) -> Option<&str> {
    let stripped = line.strip_prefix('-')?;
    Some(stripped.trim())
}

fn parse_value(raw: &str, line: usize) -> Result<Value, ParseError> {
    // Boolean
    match raw.to_lowercase().as_str() {
        "true" | "yes" => return Ok(Value::Bool(true)),
        "false" | "no" => return Ok(Value::Bool(false)),
        _ => {}
    }

    // Quoted string
    if (raw.starts_with('"') && raw.ends_with('"'))
        || (raw.starts_with('\'') && raw.ends_with('\''))
    {
        return Ok(Value::Str(raw[1..raw.len() - 1].to_string()));
    }

    // JSON-style array
    if raw.starts_with('[') {
        return parse_json_array(raw, line);
    }

    // Plain string
    Ok(Value::Str(raw.to_string()))
}

fn parse_json_array(raw: &str, line: usize) -> Result<Value, ParseError> {
    if !raw.ends_with(']') {
        return Err(ParseError::new(
            format!("invalid array syntax: {raw}"),
            line,
        ));
    }
    let inner = raw[1..raw.len() - 1].trim();
    if inner.is_empty() {
        return Ok(Value::List(vec![]));
    }

    let mut items = Vec::new();
    for part in inner.split(',') {
        let s = part.trim();
        if (s.starts_with('"') && s.ends_with('"'))
            || (s.starts_with('\'') && s.ends_with('\''))
        {
            items.push(s[1..s.len() - 1].to_string());
        } else {
            items.push(s.to_string());
        }
    }
    Ok(Value::List(items))
}
