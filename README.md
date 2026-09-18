# Boneyard

**Org-wide tech-debt radar.** Ranks internal repos by dormancy + risk + exposure + dependency depth + AI-agent surface. Emits org-wide remediation budgets.

**Status:** pre-release scaffold (2026-09-17). v0.1.0 build (MIT-licensed) was previously scaffolded at `~/Projects/boneyard/` before reuse-rebuild; that v0.1.0 source is in trash. This Apache-2.0 home is a fresh start.

## What it does

```
boneyard enrich --input hall.json --output scored.json
boneyard report  --input scored.json --policy policy.toml --output report.md
boneyard policy check --input scored.json --policy policy.toml
```

Five axes per repo: **dormancy** (real, from Necrometer), **risk**, **exposure**, **depth**, **ai_agent**. Composite Boneyard Index in [0, 100]. Bands: red / amber / green. Markdown report ranked with remediation budget in repo-weeks. TOML policy gate for CI.

## Why

- Platforms like Slack, Stripe, Atlassian, Microsoft Copilot rely on repos that quietly rot. Boneyard makes the dormancy visible before it bites.
- Same substrate family as Vigil (seance + opm), but for the org-wide layer rather than the per-repo layer.

## Commercial plane

SAML / SCIM SaaS for orgs. Slack / Teams digest alerts. Air-gapped appliance for F500 / DoD.

## License

Apache-2.0. (Previous MIT scaffold is in trash if you need to recover.)
