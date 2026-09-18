//! budget.rs — Remediation engineering budget calculator and reporter for boneyard.
use crate::cli::OutputFormat;
use crate::enrich::EnrichedHall;

pub fn emit_budget(hall: &EnrichedHall, format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => {
            let mut j = format!(
                "{{\"org\":\"{}\",\"total_remediation_weeks\":{:.1},\"repos\":[",
                hall.org_name, hall.total_remediation_weeks
            );
            for (idx, r) in hall.repos.iter().enumerate() {
                if idx > 0 {
                    j.push(',');
                }
                let rec = r.recommendation.replace('\"', "\\\"");
                j.push_str(&format!(
                    "{{\"name\":\"{}\",\"remediation_weeks\":{:.1},\"boneyard_index\":{:.1},\"recommendation\":\"{}\"}}",
                    r.repo.name, r.remediation_weeks, r.boneyard_index, rec
                ));
            }
            j.push_str("]}\n");
            j
        }
        _ => {
            let mut s = format!(
                "boneyard: remediation budget for org '{}'\n\
                 Total remediation budget: {:.1} engineer-weeks across {} repos\n\n\
                 Breakdown by repository:\n",
                hall.org_name, hall.total_remediation_weeks, hall.total_repos
            );
            for r in &hall.repos {
                s.push_str(&format!(
                    "  - {:<20} {:>5.1} weeks (Index: {:>4.1}) -> {}\n",
                    r.repo.name, r.remediation_weeks, r.boneyard_index, r.recommendation
                ));
            }
            s
        }
    }
}
