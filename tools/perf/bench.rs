// tools/perf/bench.rs — §18 bench harness for boneyard.
//
// Lives as a `#[test]` in tests/integration.rs via `include!` so it is
// exercised by `cargo test --release perf_boneyard_enrich_within_budget`.
// Honors §18: std::time only, median-of-5, line-oriented output.
//
// Budget: boneyard enrich on 1k-repo hall ≤ 1.5 s median ±25%.

use std::time::Instant;

#[test]
fn perf_boneyard_enrich_within_budget() {
    let hall = synth_hall_with_n_repos(1_000);
    let budget_ms: f64 = 1_500.0;
    let tolerance: f64 = 0.25;
    let ceiling_ms = budget_ms * (1.0 + tolerance);

    // Warm-up: prime allocator and caches, but do not measure.
    let _ = boneyard::enrich(&hall, &policy());

    // Five timed runs.
    let mut samples: Vec<f64> = Vec::with_capacity(5);
    for _ in 0..5 {
        let t = Instant::now();
        let _ = boneyard::enrich(&hall, &policy());
        let elapsed_ms = t.elapsed().as_secs_f64() * 1000.0;
        samples.push(elapsed_ms);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_ms = samples[2]; // median of 5

    let pass = median_ms <= ceiling_ms;
    println!(
        "verb=boneyard_enrich median_ms={:.3} budget_ms={:.0} pass={}",
        median_ms, budget_ms, pass
    );
    assert!(
        pass,
        "boneyard_enrich regression: median {:.1}ms > ceiling {:.1}ms",
        median_ms, ceiling_ms
    );
}

// Fixture builders — synthetic, committed (per §18-C5).
fn synth_hall_with_n_repos(n: usize) -> boneyard::Hall {
    let mut repos = Vec::with_capacity(n);
    for i in 0..n {
        repos.push(boneyard::RepoRecord {
            name: format!("org-repo-{}", i),
            is_public: i % 3 == 0,
            days_dormant: (i % 800) as u32,
            dependency_count: i % 60,
            recent_bot_commit_pct: if i % 5 == 0 { 0.8 } else { 0.1 },
            active_contributors_180d: if i % 10 == 0 { 0 } else { 2 },
        });
    }
    boneyard::Hall {
        org_name: "synth-org".to_string(),
        repos,
    }
}

fn policy() -> boneyard::Policy {
    boneyard::Policy::default()
}
