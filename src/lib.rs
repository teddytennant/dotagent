mod parser;
mod validator;

pub use parser::{parse, parse_str, AgentConfig, ParseError, Section, Value};
pub use validator::{validate, ValidationError, Severity};

#[cfg(test)]
mod tests;
