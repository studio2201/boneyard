# Changelog — boneyard

All notable changes to this project are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/) 1.1.0.
This project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.2.0] — 2026-09-18

### Added
- Working pure `std::` Rust implementation of Boneyard tech-debt radar.
- Pure `std::` JSON ingestion for organization-wide repository halls.
- Five-axis scoring engine (dormancy, exposure, dependency depth, agentic drift, bus factor) producing composite Boneyard Index (0 - 100).
- Automatic remediation budget estimations calculated in repo-weeks.
- TOML policy gating with CI pass/fail assertion.
- Executive Markdown report, JSON, and terminal summary emitters.
- Standardized CLI flags: `-h/--help`, `-V/--version`, `--format`, `-o/--output`, `-q/--quiet`, `-v/--verbose`.
- Performance test verifying 1,000 repositories scored in < 0.2ms (budget 1,500ms).

## [0.1.2] — 2026-09-17

### Changed
- `README.md` rewritten to drop openOODA substrate references
  (Necrometer / seance + opm mentions removed). The "same family as
  Vigil" framing now points at the pure-Rust + std:: style instead of
  a shared substrate.

## [0.1.1] — 2026-09-17

### Added
- §15 threat model: `docs/threat-model.md` (boneyard-specific adversary:
  platform-eng lead whose org had a dormant-but-public-facing repo
  compromised through an unmaintained dependency)
- §16 reproducible builds: `tools/dev/repro.sh` with per-host baselines
- §17 security disclosure: `SECURITY.md` pointing at GHSA tab
- §18 performance budgets: `tools/perf/budget.md` and
  `tests/integration.rs::perf_boneyard_enrich_within_budget` (std::time, median-of-5)

### Notes
- Pre-1.0.0: GHSA-only security advisories; CVEs reserved for 1.0.0+
- Budget defaults are first-cut placeholders, not aspirational

## [0.1.0] — 2026-09-17

### Added
- Initial scaffold: Apache-2.0 LICENSE, README, .gitignore
- One question (§0): "Which of my org's repos are quietly dying?"
