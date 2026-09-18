# Threat model — boneyard

## 1. Adversary
A platform-engineering lead whose org has had a dormant-but-public-facing
repo compromised through an unmaintained dependency. The attacker
typically finds these repos via GitHub search, sees the repo has not
been touched in years, and exploits a CVE in a transitive dep that no
one is monitoring. The attacker does not need a zero-day; the existing
CVEs are sufficient.

## 2. Trust boundaries
We trust: the org's repo list (the input the platform lead provides), the
last-commit timestamp as a signal of dormancy, and the CVE database
Boneyard cross-references against. We do not trust: the repo's own
self-description (a repo can claim "maintained" while being dormant),
the assumption that "no commits in N months" implies "no users" (it
does not — the repo may still be cloned by forks), or the org's awareness
of which of their repos are public.

## 3. Out of scope
Boneyard does not defend against: private repos (Boneyard only scans
public-facing repos, by design), malicious commits inside an *active*
repo (that's what Vigil §15 covers), and supply-chain attacks against
the org's *internal* package registry (Boneyard does not see internal
registries).

## 4. Residual risk
Boneyard's dormancy heuristic is "no commits in N months." An org that
has a perfectly-maintained repo in a private fork but a stale public
mirror will see the public mirror flagged. The buyer is accepting
"best-effort dormancy detection, with manual review required before
decommissioning any repo."
