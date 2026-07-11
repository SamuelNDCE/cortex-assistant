#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Ask(String),        // general AI question
    Remember(String),   // store a fact in your notes backend (text after "remember ")
    SearchVault(String),// query your notes backend
    Shutdown,           // owner-only shutdown request
    Unknown(String),    // fallback for empty/unrecognised input
}

/// Parse a transcript string into a Command using simple keyword rules.
pub fn parse_intent(transcript: &str) -> Command {
    let t = transcript.to_lowercase();
    if t.contains("shut down") || t.contains("shutdown") || t.contains("turn off") {
        Command::Shutdown
    } else if t.starts_with("remember ") {
        Command::Remember(transcript[9..].to_string())
    } else if t.contains("search") || t.contains("find in my notes") || t.contains("look up") {
        Command::SearchVault(transcript.to_string())
    } else if transcript.trim().is_empty() {
        Command::Unknown(transcript.to_string())
    } else {
        Command::Ask(transcript.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shutdown_from_shut_down() {
        assert_eq!(parse_intent("JARVIS shut down"), Command::Shutdown);
    }

    #[test]
    fn shutdown_from_turn_off() {
        assert_eq!(parse_intent("please turn off"), Command::Shutdown);
    }

    #[test]
    fn remember_strips_prefix() {
        assert_eq!(
            parse_intent("remember Samuel likes dark themes"),
            Command::Remember("Samuel likes dark themes".into()),
        );
    }

    #[test]
    fn search_vault() {
        assert!(matches!(parse_intent("search my notes for JARVIS"), Command::SearchVault(_)));
    }

    #[test]
    fn empty_is_unknown() {
        assert!(matches!(parse_intent("   "), Command::Unknown(_)));
    }

    #[test]
    fn general_question_is_ask() {
        assert!(matches!(parse_intent("What is the weather today?"), Command::Ask(_)));
    }

    #[test]
    fn remember_case_sensitive_prefix() {
        // "remember " is lowercase check — transcript must start with "remember " (lowercase)
        // This checks the intent parser handles lowercase "remember" correctly
        assert!(matches!(parse_intent("remember to buy milk"), Command::Remember(_)));
    }
}
