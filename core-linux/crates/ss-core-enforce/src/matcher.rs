use ss_core_model::evaluator::ProcessCandidate;
use ss_core_model::policy::{Policy, Rule};

/// Returns `true` if at least one `AppBlocklist` or `AppAllowlist` rule in any
/// of the given policies references this process (via at least one matcher).
///
/// Useful for deciding whether to open a cgroup scope and monitor a process.
#[must_use]
pub fn match_process(policies: &[Policy], p: &ProcessCandidate) -> bool {
    for pol in policies {
        for rule in &pol.rules {
            let (Rule::AppBlocklist { matchers } | Rule::AppAllowlist { matchers }) = rule else {
                continue;
            };
            for m in matchers {
                if m.matches(p.content_hash.as_deref(), &p.basename, &p.path) {
                    return true;
                }
            }
        }
    }
    false
}
