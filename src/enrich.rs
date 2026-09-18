//! enrich.rs — Five-axis tech debt and dormancy scoring.
//! Calculates Boneyard Index (0.0 to 100.0) and remediation budgets.

use crate::hall::{Hall, RepoRecord};
use std::fmt;

#[derive(Debug, Clone)]
pub struct EnrichedRepo {
    pub repo: RepoRecord,
    pub dormancy_score: f32,
    pub exposure_score: f32,
    pub dependency_score: f32,
    pub agentic_score: f32,
    pub bus_factor_score: f32,
    pub boneyard_index: f32,
    pub remediation_weeks: f32,
    pub recommendation: String,
}

#[derive(Debug, Clone)]
pub struct EnrichedHall {
    pub org_name: String,
    pub total_repos: usize,
    pub avg_boneyard_index: f32,
    pub total_remediation_weeks: f32,
    pub critical_count: usize,
    pub high_count: usize,
    pub repos: Vec<EnrichedRepo>,
}

#[derive(Debug)]
pub struct EnrichError(pub String);

impl fmt::Display for EnrichError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Enrich error: {}", self.0)
    }
}

impl std::error::Error for EnrichError {}

pub fn score_repo(r: &RepoRecord) -> EnrichedRepo {
    // 1. Dormancy (0 - 30 pts)
    let dormancy_score = ((r.days_dormant as f32 / 365.0) * 30.0).clamp(0.0, 30.0);

    // 2. Exposure (0 - 20 pts)
    let exposure_score = if r.is_public { 20.0 } else { 5.0 };

    // 3. Dependency Depth (0 - 20 pts)
    let dependency_score = ((r.dependency_count as f32 / 50.0) * 20.0).clamp(0.0, 20.0);

    // 4. Agentic Drift (0 - 15 pts)
    let agentic_score = (r.recent_bot_commit_pct * 15.0).clamp(0.0, 15.0);

    // 5. Bus Factor (0 - 15 pts)
    let bus_factor_score = match r.active_contributors_180d {
        0 => 15.0,
        1 => 12.0,
        2 => 6.0,
        _ => 2.0,
    };

    let boneyard_index = (dormancy_score + exposure_score + dependency_score + agentic_score + bus_factor_score)
        .clamp(0.0, 100.0);

    let (remediation_weeks, recommendation) = if boneyard_index >= 75.0 {
        (0.2, "Archive repository (immediate dead-weight cleanup)".to_string())
    } else if boneyard_index >= 55.0 {
        (2.5, "Refactor & upgrade dependencies; audit agent changes".to_string())
    } else if boneyard_index >= 35.0 {
        (1.0, "Routine maintenance and dependency bump".to_string())
    } else {
        (0.0, "Active and healthy; no action required".to_string())
    };

    EnrichedRepo {
        repo: r.clone(),
        dormancy_score,
        exposure_score,
        dependency_score,
        agentic_score,
        bus_factor_score,
        boneyard_index,
        remediation_weeks,
        recommendation,
    }
}

pub fn enrich_hall(hall: &Hall) -> Result<EnrichedHall, EnrichError> {
    let mut repos = Vec::with_capacity(hall.repos.len());
    let mut total_index = 0.0;
    let mut total_weeks = 0.0;
    let mut critical_count = 0;
    let mut high_count = 0;

    for r in &hall.repos {
        let scored = score_repo(r);
        total_index += scored.boneyard_index;
        total_weeks += scored.remediation_weeks;
        if scored.boneyard_index >= 75.0 {
            critical_count += 1;
        } else if scored.boneyard_index >= 55.0 {
            high_count += 1;
        }
        repos.push(scored);
    }

    // Sort descending by Boneyard Index (highest tech debt first)
    repos.sort_by(|a, b| b.boneyard_index.partial_cmp(&a.boneyard_index).unwrap_or(std::cmp::Ordering::Equal));

    let count = repos.len();
    let avg_boneyard_index = if count > 0 { total_index / count as f32 } else { 0.0 };

    Ok(EnrichedHall {
        org_name: hall.org_name.clone(),
        total_repos: count,
        avg_boneyard_index,
        total_remediation_weeks: total_weeks,
        critical_count,
        high_count,
        repos,
    })
}
