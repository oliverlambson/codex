use super::*;
use pretty_assertions::assert_eq;
use std::ffi::OsString;

fn strategy(
    preference: HistoryInsertionPreference,
    terminal_kind: TerminalKind,
) -> HistoryInsertionStrategy {
    HistoryInsertionStrategy {
        preference,
        terminal_kind,
    }
}

#[test]
fn parses_history_insertion_preference() {
    assert_eq!(parse_preference(None), Ok(HistoryInsertionPreference::Auto));
    assert_eq!(
        parse_preference(Some("")),
        Ok(HistoryInsertionPreference::Auto)
    );
    assert_eq!(
        parse_preference(Some("append")),
        Ok(HistoryInsertionPreference::Append)
    );
    assert_eq!(parse_preference(Some("invalid")), Err(()));
}

#[test]
fn invalid_environment_value_falls_back_to_auto() {
    assert_eq!(
        preference_from_env_result(Ok("invalid".to_string())),
        HistoryInsertionPreference::Auto
    );
}

#[test]
fn append_environment_value_selects_append() {
    assert_eq!(
        preference_from_env_result(Ok("append".to_string())),
        HistoryInsertionPreference::Append
    );
}

#[test]
fn non_unicode_environment_value_falls_back_to_auto() {
    assert_eq!(
        preference_from_env_result(Err(VarError::NotUnicode(OsString::from("invalid")))),
        HistoryInsertionPreference::Auto
    );
}

#[test]
fn forced_append_applies_to_pre_wrapped_history() {
    assert_eq!(
        strategy(HistoryInsertionPreference::Append, TerminalKind::Direct)
            .mode_for(HistoryLineWrapPolicy::PreWrap),
        InsertHistoryMode::Append
    );
}

#[test]
fn forced_append_applies_to_terminal_wrapped_history() {
    assert_eq!(
        strategy(HistoryInsertionPreference::Append, TerminalKind::Direct)
            .mode_for(HistoryLineWrapPolicy::Terminal),
        InsertHistoryMode::Append
    );
}

#[test]
fn direct_terminals_use_standard_pre_wrapped_history_by_default() {
    assert_eq!(
        strategy(HistoryInsertionPreference::Auto, TerminalKind::Direct)
            .mode_for(HistoryLineWrapPolicy::PreWrap),
        InsertHistoryMode::Standard
    );
}

#[test]
fn direct_terminals_use_standard_terminal_wrapped_history_by_default() {
    assert_eq!(
        strategy(HistoryInsertionPreference::Auto, TerminalKind::Direct)
            .mode_for(HistoryLineWrapPolicy::Terminal),
        InsertHistoryMode::Standard
    );
}

#[test]
fn zellij_uses_standard_pre_wrapped_history_by_default() {
    assert_eq!(
        strategy(HistoryInsertionPreference::Auto, TerminalKind::Zellij)
            .mode_for(HistoryLineWrapPolicy::PreWrap),
        InsertHistoryMode::Standard
    );
}

#[test]
fn zellij_uses_append_for_terminal_wrapped_history_by_default() {
    assert_eq!(
        strategy(HistoryInsertionPreference::Auto, TerminalKind::Zellij)
            .mode_for(HistoryLineWrapPolicy::Terminal),
        InsertHistoryMode::Append
    );
}
