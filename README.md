# Boneyard

[![CI](https://github.com/studio2201/boneyard/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/studio2201/boneyard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/badge/version-v0.2.5-blue.svg)](https://github.com/studio2201/boneyard/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Pure std::](https://img.shields.io/badge/pure-std%3A%3A-success.svg)](https://studio2201.com)
[![Reproducible](https://img.shields.io/badge/reproducible-OK-brightgreen.svg)](tools/dev/repro.sh)
[![Max LOC](https://img.shields.io/badge/max%20LOC-%E2%89%A4256-brightgreen.svg)](https://studio2201.com)

[![Boneyard Index](https://img.shields.io/badge/boneyard%20index-0%2F100-brightgreen.svg)](https://studio2201.com/boneyard)
[![Remediation Budget](https://img.shields.io/badge/debt%20budget-0%20repo--wks-brightgreen.svg)](https://studio2201.com/boneyard)
[![Radar Scoring](https://img.shields.io/badge/radar-5--axis%20scoring-blue.svg)](https://studio2201.com/boneyard)
[![Policy Gate](https://img.shields.io/badge/policy%20gate-PASSED-brightgreen.svg)](https://studio2201.com/boneyard)

**Org-wide tech-debt radar.** Ranks internal repos by dormancy + risk + exposure + dependency depth + AI-agent surface. Emits org-wide remediation budgets in repo-weeks.

## Why This Matters & Authoritative Research

### 1. The $300 Billion Technical Debt Crisis
Large engineering organizations maintain hundreds or thousands of Git repositories. Over time, repositories silently rot: dependencies fall years behind, original maintainers leave (bus factor collapse), and automated bots churn untested commits.
- **[Stripe Developer Coefficient Report](https://stripe.com/reports/developer-coefficient-2018)**: Landmark empirical research establishing that bad code and technical debt cost the global economy over $300 billion annually, with engineers losing 33% of their productive time maintaining legacy debt.
- **[ACM Empirical Software Engineering](https://dl.acm.org/doi/10.1145/3377811.3380385)**: Quantifies the compounding risks of unmonitored repository dormancy and architectural decay across enterprise code catalogs.
- **[Gartner Technical Debt Management Research](https://www.gartner.com/smarterwithgartner/how-to-manage-technical-debt)**: Urges engineering leadership to establish clear, measurable remediation budgets to prevent systemic operational paralysis.

## How It Works Under the Hood

1. **Hall Ingestion (`src/hall.rs`)**: Reads organization-wide repository catalogs (`hall.json`) mapping commit history, contributor counts, public/private exposure, and dependency depth.
2. **Five-Axis Scoring Engine (`src/enrich.rs`)**:
   - **Dormancy (0–30 pts)**: Days since last human commit scaled over 365 days.
   - **Exposure (0–20 pts)**: Public Internet surface area (20 pts) vs internal service (5 pts).
   - **Dependency Depth (0–20 pts)**: Nested dependency tree complexity.
   - **Agentic Drift (0–15 pts)**: Ratio of automated bot commits versus human code review.
   - **Bus Factor (0–15 pts)**: Active human contributors in the last 180 days (0 contributors = 15 pts).
3. **Remediation Budgeting**: Translates the composite Boneyard Index (0.0 to 100.0) into actionable engineering allocations expressed in **repo-weeks**.
4. **Automated CI Policy Gating (`src/policy.rs`)**: Evaluates repository debt against TOML compliance thresholds.

## Quick Start

```bash
# Install via studio2201 installer
curl -fsSL https://studio2201.com/install.sh | sh -s boneyard

# Enrich hall metadata and compute Boneyard Index
boneyard enrich --input hall.json --output scored.json

# Calculate org remediation budget
boneyard budget --input hall.json

# Run system diagnostics
boneyard doctor
```

## GitHub Action Usage

Enrich repository metadata and generate debt reports in CI:

```yaml
- name: Boneyard Tech-Debt Radar
  uses: studio2201/boneyard@master
  with:
    input: 'hall.json'
    output: 'scored.json'
```

## CLI Commands

- `boneyard enrich` (or `scan`) — Score repository hall
- `boneyard budget` — Calculate remediation budgets
- `boneyard report` — Generate markdown audit report
- `boneyard policy check` — Evaluate against threshold policy
- `boneyard doctor` — Run 7-point system diagnostics
- `boneyard update` / `boneyard upgrade` — Self-update binary
- `boneyard -h` / `--help` — Show help
- `boneyard -V` / `--version` — Show version

## Badges & Status

Display your organization's technical debt score and remediation budget:

```markdown
<!-- Boneyard Tech-Debt Index Badge -->
[![Boneyard Index](https://img.shields.io/badge/boneyard%20index-0%2F100-brightgreen.svg)](https://studio2201.com/boneyard)

<!-- Remediation Budget Shield -->
[![Debt Budget](https://img.shields.io/badge/debt%20budget-0%20repo--wks-brightgreen.svg)](https://studio2201.com/boneyard)
```

## License

Apache-2.0.
