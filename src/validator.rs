use crate::parser::AgentConfig;
use std::fmt;

const VALID_MODES: &[&str] = &["autonomous", "interactive", "supervised", "passive"];

const KNOWN_SECTIONS: &[&str] = &[
    "identity",
    "constraints",
    "memory",
    "scope",
    "schedule",
    "tools",
    "environment",
    "permissions",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub severity: Severity,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.field, self.message)
    }
}

pub fn validate(config: &AgentConfig) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // Name is required
    if config.name.is_none() {
        errors.push(ValidationError {
            field: "name".into(),
            message: "agent name is required".into(),
            severity: Severity::Error,
        });
    }

    // Mode must be valid if set
    if let Some(mode) = &config.mode {
        if !VALID_MODES.contains(&mode.as_str()) {
            errors.push(ValidationError {
                field: "mode".into(),
                message: format!(
                    "invalid mode '{mode}', must be one of: {}",
                    VALID_MODES.join(", ")
                ),
                severity: Severity::Error,
            });
        }
    }

    // Warn on unknown sections
    for section_name in config.sections.keys() {
        if !KNOWN_SECTIONS.contains(&section_name.as_str()) {
            errors.push(ValidationError {
                field: format!("[{section_name}]"),
                message: format!("unknown section '{section_name}'"),
                severity: Severity::Warning,
            });
        }
    }

    // Schedule: can't have both interval and cron
    if let Some(schedule) = config.section("schedule") {
        if schedule.get("interval").is_some() && schedule.get("cron").is_some() {
            errors.push(ValidationError {
                field: "[schedule]".into(),
                message: "cannot specify both 'interval' and 'cron'".into(),
                severity: Severity::Error,
            });
        }
    }

    errors
}
