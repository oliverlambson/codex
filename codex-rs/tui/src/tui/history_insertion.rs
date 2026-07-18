use std::env::VarError;

use crate::insert_history::HistoryLineWrapPolicy;
use crate::insert_history::InsertHistoryMode;

const HISTORY_MODE_ENV_VAR: &str = "CODEX_TUI_HISTORY_MODE";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HistoryInsertionPreference {
    Auto,
    Append,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TerminalKind {
    Direct,
    Zellij,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct HistoryInsertionStrategy {
    preference: HistoryInsertionPreference,
    terminal_kind: TerminalKind,
}

impl HistoryInsertionStrategy {
    pub(super) fn detect() -> Self {
        let preference = preference_from_env_result(std::env::var(HISTORY_MODE_ENV_VAR));
        let terminal_kind = if codex_terminal_detection::terminal_info().is_zellij() {
            TerminalKind::Zellij
        } else {
            TerminalKind::Direct
        };
        Self {
            preference,
            terminal_kind,
        }
    }

    pub(super) fn mode_for(self, wrap_policy: HistoryLineWrapPolicy) -> InsertHistoryMode {
        match (self.preference, self.terminal_kind, wrap_policy) {
            (HistoryInsertionPreference::Append, _, _) => InsertHistoryMode::Append,
            (
                HistoryInsertionPreference::Auto,
                TerminalKind::Zellij,
                HistoryLineWrapPolicy::Terminal,
            ) => InsertHistoryMode::Append,
            (
                HistoryInsertionPreference::Auto,
                TerminalKind::Direct,
                HistoryLineWrapPolicy::Terminal,
            )
            | (HistoryInsertionPreference::Auto, _, HistoryLineWrapPolicy::PreWrap) => {
                InsertHistoryMode::Standard
            }
        }
    }
}

fn preference_from_env_result(value: Result<String, VarError>) -> HistoryInsertionPreference {
    match value {
        Ok(value) => match parse_preference(Some(&value)) {
            Ok(preference) => preference,
            Err(()) => {
                tracing::warn!(
                    value,
                    "invalid {HISTORY_MODE_ENV_VAR} value; using automatic history insertion"
                );
                HistoryInsertionPreference::Auto
            }
        },
        Err(VarError::NotPresent) => HistoryInsertionPreference::Auto,
        Err(VarError::NotUnicode(_)) => {
            tracing::warn!(
                "non-Unicode {HISTORY_MODE_ENV_VAR} value; using automatic history insertion"
            );
            HistoryInsertionPreference::Auto
        }
    }
}

fn parse_preference(value: Option<&str>) -> Result<HistoryInsertionPreference, ()> {
    match value {
        None | Some("") => Ok(HistoryInsertionPreference::Auto),
        Some("append") => Ok(HistoryInsertionPreference::Append),
        Some(_) => Err(()),
    }
}

#[cfg(test)]
#[path = "history_insertion_tests.rs"]
mod tests;
