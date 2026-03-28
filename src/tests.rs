use crate::parser::*;
use crate::validator::*;

// ── Parser tests ──

#[test]
fn parse_minimal() {
    let config = parse_str("name: test\n").unwrap();
    assert_eq!(config.name.as_deref(), Some("test"));
}

#[test]
fn parse_all_headers() {
    let text = "name: my-agent\nextends: base\nmode: autonomous\ndescription: A test agent\n";
    let config = parse_str(text).unwrap();
    assert_eq!(config.name.as_deref(), Some("my-agent"));
    assert_eq!(config.extends.as_deref(), Some("base"));
    assert_eq!(config.mode.as_deref(), Some("autonomous"));
    assert_eq!(config.description.as_deref(), Some("A test agent"));
}

#[test]
fn parse_section_with_kv() {
    let text = "name: test\n\n[identity]\nauthor = \"Alice\"\nrole = engineer\n";
    let config = parse_str(text).unwrap();
    let section = config.section("identity").unwrap();
    assert_eq!(section.get_str("author"), Some("Alice"));
    assert_eq!(section.get_str("role"), Some("engineer"));
}

#[test]
fn parse_section_with_list() {
    let text = "name: test\n\n[constraints]\n- be safe\n- be helpful\n";
    let config = parse_str(text).unwrap();
    let section = config.section("constraints").unwrap();
    assert_eq!(
        section.get_list("_items"),
        Some(vec!["be safe".to_string(), "be helpful".to_string()].as_slice())
    );
}

#[test]
fn parse_boolean_values() {
    let text = "name: test\n\n[memory]\nread-on-start = true\nwrite-on-end = false\n";
    let config = parse_str(text).unwrap();
    let section = config.section("memory").unwrap();
    assert_eq!(section.get_bool("read-on-start"), Some(true));
    assert_eq!(section.get_bool("write-on-end"), Some(false));
}

#[test]
fn parse_json_array_value() {
    let text = "name: test\n\n[scope]\npaths = [\"/a\", \"/b\"]\n";
    let config = parse_str(text).unwrap();
    let section = config.section("scope").unwrap();
    assert_eq!(
        section.get_list("paths"),
        Some(vec!["/a".to_string(), "/b".to_string()].as_slice())
    );
}

#[test]
fn parse_comments_ignored() {
    let text = "# comment\nname: test\n# another\n\n[constraints]\n# skip\n- rule one\n";
    let config = parse_str(text).unwrap();
    assert_eq!(config.name.as_deref(), Some("test"));
    let section = config.section("constraints").unwrap();
    assert_eq!(
        section.get_list("_items"),
        Some(vec!["rule one".to_string()].as_slice())
    );
}

#[test]
fn parse_blank_lines_ignored() {
    let text = "\n\nname: test\n\n\n[identity]\n\nauthor = Bob\n\n";
    let config = parse_str(text).unwrap();
    assert_eq!(config.name.as_deref(), Some("test"));
    assert_eq!(
        config.section("identity").unwrap().get_str("author"),
        Some("Bob")
    );
}

#[test]
fn parse_multiple_sections() {
    let text =
        "name: test\n\n[identity]\nauthor = A\n\n[constraints]\n- rule\n\n[scope]\npaths = [\"/x\"]\n";
    let config = parse_str(text).unwrap();
    assert!(config.has_section("identity"));
    assert!(config.has_section("constraints"));
    assert!(config.has_section("scope"));
}

#[test]
fn parse_quoted_string_strips_quotes() {
    let text = "name: test\n\n[identity]\nauthor = \"Alice Bob\"\n";
    let config = parse_str(text).unwrap();
    assert_eq!(
        config.section("identity").unwrap().get_str("author"),
        Some("Alice Bob")
    );
}

#[test]
fn parse_unknown_header_errors() {
    let result = parse_str("name: test\nfoo: bar\n");
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("unknown top-level header"));
}

#[test]
fn parse_bad_syntax_errors() {
    let result = parse_str("name: test\ngarbage line\n");
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("unexpected"));
}

#[test]
fn parse_content_outside_section_errors() {
    let result = parse_str("name: test\n- stray list item\n");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .message
        .contains("unexpected content outside section"));
}

#[test]
fn parse_empty_array() {
    let text = "name: test\n\n[tools]\nallowed = []\n";
    let config = parse_str(text).unwrap();
    let section = config.section("tools").unwrap();
    assert_eq!(section.get_list("allowed"), Some(Vec::new().as_slice()));
}

// ── File parsing tests ──

#[test]
fn parse_valid_full_file() {
    let config = parse("tests/fixtures/valid/full.agent").unwrap();
    assert_eq!(config.name.as_deref(), Some("test-agent"));
    assert_eq!(config.extends.as_deref(), Some("base"));
    assert_eq!(config.mode.as_deref(), Some("autonomous"));
    assert!(config.source.is_some());
    assert!(config.has_section("identity"));
    assert!(config.has_section("constraints"));
    assert!(config.has_section("memory"));
    assert!(config.has_section("scope"));
    assert!(config.has_section("schedule"));
}

#[test]
fn parse_valid_minimal_file() {
    let config = parse("tests/fixtures/valid/minimal.agent").unwrap();
    assert_eq!(config.name.as_deref(), Some("minimal"));
    assert!(config.sections.is_empty());
}

#[test]
fn parse_valid_comments_file() {
    let config = parse("tests/fixtures/valid/comments.agent").unwrap();
    assert_eq!(config.name.as_deref(), Some("commented"));
    assert_eq!(
        config
            .section("constraints")
            .unwrap()
            .get_list("_items"),
        Some(vec!["be safe".to_string(), "be helpful".to_string()].as_slice())
    );
}

#[test]
fn parse_missing_file_errors() {
    let result = parse("/nonexistent/path.agent");
    assert!(result.is_err());
}

#[test]
fn parse_bad_syntax_file_errors() {
    let result = parse("tests/fixtures/invalid/bad_syntax.agent");
    assert!(result.is_err());
}

// ── Validator tests ──

#[test]
fn validate_valid_config() {
    let config = parse_str("name: test\nmode: autonomous\n").unwrap();
    let errors = validate(&config);
    assert!(errors.is_empty());
}

#[test]
fn validate_missing_name() {
    let config = parse_str("mode: autonomous\n").unwrap();
    let errors = validate(&config);
    assert!(errors.iter().any(|e| e.field == "name" && e.severity == Severity::Error));
}

#[test]
fn validate_invalid_mode() {
    let config = parse_str("name: test\nmode: yolo\n").unwrap();
    let errors = validate(&config);
    assert!(errors.iter().any(|e| e.field == "mode" && e.severity == Severity::Error));
}

#[test]
fn validate_all_valid_modes() {
    for mode in &["autonomous", "interactive", "supervised", "passive"] {
        let config = parse_str(&format!("name: test\nmode: {mode}\n")).unwrap();
        let errors = validate(&config);
        assert!(
            !errors.iter().any(|e| e.field == "mode"),
            "mode '{mode}' should be valid"
        );
    }
}

#[test]
fn validate_unknown_section_warns() {
    let config = parse_str("name: test\n\n[banana]\nfoo = bar\n").unwrap();
    let errors = validate(&config);
    assert!(errors
        .iter()
        .any(|e| e.severity == Severity::Warning && e.message.contains("banana")));
}

#[test]
fn validate_known_sections_no_warning() {
    let config =
        parse_str("name: test\n\n[identity]\nauthor = A\n\n[constraints]\n- rule\n").unwrap();
    let errors = validate(&config);
    assert!(!errors.iter().any(|e| e.severity == Severity::Warning));
}

#[test]
fn validate_schedule_conflict() {
    let text = "name: test\n\n[schedule]\ninterval = 5m\ncron = \"0 * * * *\"\n";
    let config = parse_str(text).unwrap();
    let errors = validate(&config);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("interval") && e.message.contains("cron")));
}

#[test]
fn validate_no_mode_is_valid() {
    let config = parse_str("name: test\n").unwrap();
    let errors = validate(&config);
    assert!(errors.is_empty());
}
