//! hall.rs — Repository hall and metadata structures.
//! Hand-rolled JSON parsing in pure std:: for org-wide ingestion.

use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct RepoRecord {
    pub name: String,
    pub is_public: bool,
    pub days_dormant: u32,
    pub dependency_count: usize,
    pub recent_bot_commit_pct: f32,
    pub active_contributors_180d: usize,
}

#[derive(Debug, Clone)]
pub struct Hall {
    pub org_name: String,
    pub repos: Vec<RepoRecord>,
}

#[derive(Debug)]
pub enum HallError {
    Io(std::io::Error),
    Parse(String),
}

impl fmt::Display for HallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HallError::Io(e) => write!(f, "I/O error: {}", e),
            HallError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for HallError {}

impl From<std::io::Error> for HallError {
    fn from(e: std::io::Error) -> Self {
        HallError::Io(e)
    }
}

pub fn load_hall_file(path: &Path) -> Result<Hall, HallError> {
    let raw = fs::read_to_string(path)?;
    parse_hall_json(&raw)
}

pub fn parse_hall_json(raw: &str) -> Result<Hall, HallError> {
    let mut repos = Vec::new();
    let mut org_name = "org".to_string();

    for block in raw.split('{') {
        if !block.contains('}') { continue; }
        let content = &block[..block.find('}').unwrap_or(block.len())];

        let mut name = None;
        let mut is_public = true;
        let mut days_dormant = 0;
        let mut dep_count = 0;
        let mut bot_pct = 0.0;
        let mut contribs = 1;

        for field in content.split(&['\n', ','][..]) {
            let t = field.trim().trim_matches(',');
            if let Some(pos) = t.find(':') {
                let k = t[..pos].trim().trim_matches('"');
                let v = t[pos + 1..].trim().trim_matches('"');
                match k {
                    "org" | "org_name" => org_name = v.to_string(),
                    "name" | "repo" => name = Some(v.to_string()),
                    "is_public" | "public" => is_public = v == "true",
                    "days_dormant" | "dormant_days" => days_dormant = v.parse::<u32>().unwrap_or(0),
                    "dependency_count" | "deps" => dep_count = v.parse::<usize>().unwrap_or(0),
                    "recent_bot_commit_pct" | "bot_pct" => bot_pct = v.parse::<f32>().unwrap_or(0.0),
                    "active_contributors_180d" | "contributors" => contribs = v.parse::<usize>().unwrap_or(1),
                    _ => {}
                }
            }
        }

        if let Some(n) = name {
            repos.push(RepoRecord {
                name: n,
                is_public,
                days_dormant,
                dependency_count: dep_count,
                recent_bot_commit_pct: bot_pct,
                active_contributors_180d: contribs,
            });
        }
    }

    Ok(Hall { org_name, repos })
}
