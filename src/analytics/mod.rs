//! Metrics are currently scoped to each Claude Code session.
pub fn print_stats_help() {
    println!("CMR statistics are session-scoped: /cmr stats inside Claude Code.");
    println!("For RTK command savings, run: rtk gain");
    println!("These counters do not establish subscription quota savings.");
}
