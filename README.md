# Boneyard

[![CI](https://github.com/studio2201/boneyard/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/studio2201/boneyard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/badge/version-v0.2.3-blue.svg)](https://github.com/studio2201/boneyard/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Pure std::](https://img.shields.io/badge/pure-std%3A%3A-success.svg)](https://studio2201.com)
[![Reproducible](https://img.shields.io/badge/reproducible-OK-brightgreen.svg)](tools/dev/repro.sh)
[![Max LOC](https://img.shields.io/badge/max%20LOC-%E2%89%A4256-brightgreen.svg)](https://studio2201.com)

**Org-wide tech-debt radar.** Ranks internal repos by dormancy + risk + exposure + dependency depth + AI-agent surface. Emits org-wide remediation budgets in repo-weeks.

## Quick Start

```bash
# Install via studio2201 installer
curl -fsSL https://studio2201.com/install.sh | sh -s boneyard

# Enrich hall metadata and compute Boneyard Index
boneyard enrich --input hall.json --output scored.json

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

## What it does

Five axes per repo: **dormancy**, **risk**, **exposure**, **depth**, and **ai_agent**. Composite Boneyard Index in [0, 100]. Emits Markdown reports with remediation budgets in repo-weeks and TOML policy gates for CI.

## CLI Commands

- `boneyard enrich` (or `scan`) — Score repository hall
- `boneyard budget` — Calculate remediation budgets
- `boneyard report` — Generate markdown audit report
- `boneyard policy check` — Evaluate against threshold policy
- `boneyard doctor` — Run 7-point system diagnostics
- `boneyard update` / `boneyard upgrade` — Self-update binary
- `boneyard -h` / `--help` — Show help
- `boneyard -V` / `--version` — Show version

## Why

- Platforms like Slack, Stripe, Atlassian, Microsoft Copilot rely on repos that quietly rot. Boneyard makes the dormancy visible before it bites.
- Pure Rust, `std::` only. Zero crates.io dependencies. Strictly <= 256 LOC per source file.

## License

Apache-2.0.
