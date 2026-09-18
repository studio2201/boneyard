//! policy.rs — TOML policy parsing and compliance gating.
//! Evaluates enriched repositories against tech-debt thresholds in pure std::.

use crate::enrich::EnrichedHall;

#[derive(Debug, Clone)]
pub struct Policy {
    pub max_avg_boneyard_index: f32,
    pub max_critical_repos: usize,
    pub max_remediation_weeks: f32,
}

impl Default for Policy {
    fn default() -> Self {
        Policy {
            max_avg_boneyard_index: 50.0,
            max_critical_repos: 0,
            max_remediation_weeks: 15.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PolicyVerdict {
    pub passed: bool,
    pub violations: Vec<String>,
}

pub fn parse_policy_toml(content: &str) -> Policy {
    let mut pol = Policy::default();
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with('#') || t.is_empty() { continue; }
        if let Some(pos) = t.find('=') {
            let k = t[..pos].trim();
            let v = t[pos + 1..].trim().trim_matches('"');
            match k {
                "max_avg_boneyard_index" | "max_avg_index" => {
                    if let Ok(val) = v.parse::<f32>() { pol.max_avg_boneyard_index = val; }
                }
                "max_critical_repos" | "max_critical" => {
                    if let Ok(val) = v.parse::<usize>() { pol.max_critical_repos = val; }
                }
                "max_remediation_weeks" | "max_weeks" => {
                    if let Ok(val) = v.parse::<f32>() { pol.max_remediation_weeks = val; }
                }
                _ => {}
            }
        }
    }
    pol
}

pub fn evaluate(hall: &EnrichedHall, policy: &Policy) -> PolicyVerdict {
    let mut violations = Vec::new();

    if hall.avg_boneyard_index > policy.max_avg_boneyard_index {
        violations.push(format!(
            "Org average Boneyard Index {:.1} exceeds maximum allowed {:.1}",
            hall.avg_boneyard_index, policy.max_avg_boneyard_index
        ));
    }

    if hall.critical_count > policy.max_critical_repos {
        violations.push(format!(
            "Critical repository count {} exceeds threshold {}",
            hall.critical_count, policy.max_critical_repos
        ));
    }

    if hall.total_remediation_weeks > policy.max_remediation_weeks {
        violations.push(format!(
            "Total remediation backlog of {:.1} repo-weeks exceeds cap of {:.1}",
            hall.total_remediation_weeks, policy.max_remediation_weeks
        ));
    }

    let passed = violations.is_empty();
    PolicyVerdict { passed, violations }
}
