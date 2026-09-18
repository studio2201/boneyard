# Boneyard

**Org-wide tech-debt radar.** Ranks internal repos by dormancy + risk + exposure + dependency depth + AI-agent surface. Emits org-wide remediation budgets.

**Status:** v0.2.0 release (2026-09-18).

## What it does

```
boneyard enrich --input hall.json --output scored.json
boneyard report  --input scored.json --policy policy.toml --output report.md
boneyard policy check --input scored.json --policy policy.toml
```

Five axes per repo: **dormancy**, **risk**, **exposure**, **depth**, **ai_agent**. Composite Boneyard Index in [0, 100]. Bands: red / amber / green. Markdown report ranked with remediation budget in repo-weeks. TOML policy gate for CI.

## Why

- Platforms like Slack, Stripe, Atlassian, Microsoft Copilot rely on repos that quietly rot. Boneyard makes the dormancy visible before it bites.
- Same family as Vigil — pure Rust, `std::` only, ≤ 256 LoC per file — but for the org-wide layer rather than the per-repo layer.

## Commercial plane

SAML / SCIM SaaS for orgs. Slack / Teams digest alerts. Air-gapped appliance for F500 / DoD.

## License

Apache-2.0. (Previous MIT scaffold is in trash if you need to recover.)
