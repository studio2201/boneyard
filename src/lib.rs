//! lib.rs — Boneyard: Org-wide tech-debt radar and repo scoring engine.
//! Pure std:: Rust (Edition 2021). Zero external dependencies.

pub mod enrich;
pub mod hall;
pub mod policy;
pub mod report;

pub use enrich::{enrich_hall, score_repo, EnrichedHall, EnrichedRepo, EnrichError};
pub use hall::{load_hall_file, parse_hall_json, Hall, RepoRecord};
pub use policy::{evaluate, parse_policy_toml, Policy, PolicyVerdict};
pub use report::{emit_json, emit_markdown, emit_text};

/// Enriches a repository hall by calculating five-axis metrics and remediation budgets.
pub fn enrich(hall: &Hall, _policy: &Policy) -> Result<EnrichedHall, EnrichError> {
    enrich_hall(hall)
}
