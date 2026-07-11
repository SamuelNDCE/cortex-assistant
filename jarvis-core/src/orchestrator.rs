use crate::commands::{Command, parse_intent};

pub struct Orchestrator;

impl Orchestrator {
    pub fn new() -> Self { Self }

    /// Route a transcript string to a Command.
    pub fn route(&self, transcript: &str) -> Command {
        parse_intent(transcript)
    }
}

impl Default for Orchestrator {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_general_to_ask() {
        let orch = Orchestrator::new();
        assert!(matches!(orch.route("tell me about fusion"), Command::Ask(_)));
    }

    #[test]
    fn routes_shutdown() {
        let orch = Orchestrator::new();
        assert_eq!(orch.route("shutdown"), Command::Shutdown);
    }
}
