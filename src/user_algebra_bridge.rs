//! A thin wrapper around the `user_algebra` crate.
//!
//! This file exists to keep UI code (`src/ui/*`) focused on presentation/state, and
//! keep the "business logic wiring" in one place.

/// A leaf node (number) the UI can display and allow the user to select.
#[derive(Clone, Debug, PartialEq)]
pub struct LeafChip {
    pub id: usize,
    pub value: i128,
}

/// Result of trying to apply a move (solve two leaf ids).
pub enum SolveOutcome {
    Applied(user_algebra::SolveOk),
    Rejected(user_algebra::SolveError),
}

/// Parse a string into an arena using the logic crate.
///
/// NOTE: `user_algebra::convert_str_to_expression_usable` currently unwraps parse errors internally.
pub fn parse_expression(input: String) -> user_algebra::Arena {
    user_algebra::convert_str_to_expression_usable(input)
}

/// Apply the "solve two leaves" move via the logic crate.
pub fn solve_pair(
    arena: &mut user_algebra::Arena,
    id1: usize,
    id2: usize,
) -> SolveOutcome {
    match user_algebra::solve_leaves_api(arena, id1, id2) {
        Ok(ok) => SolveOutcome::Applied(ok),
        Err(err) => SolveOutcome::Rejected(err),
    }
}

/// Extract all leaf nodes (numbers) from an arena.
///
/// This is used by the UI to show the clickable list of number chips.
///
/// Current limitation:
/// - `Arena` does not expose `len()` or an iterator over nodes/leaves.
/// - Even with `Node` being public, we cannot traverse nodes without a way to enumerate ids.
///
/// Workaround:
/// - We parse the `Debug` output for `Arena`, which includes the internal `nodes: [...]`.
///
/// If you expose either:
/// - `pub fn len(&self) -> usize`
/// - or `pub fn nodes(&self) -> &[Node]`
/// we can replace this with a real traversal and drop the string parsing.
pub fn leaf_chips_from_arena_debug(arena: &user_algebra::Arena) -> Vec<LeafChip> {
    let dbg = format!("{arena:?}");
    let mut out = Vec::new();

    // State machine: walk debug and count node entries inside "nodes: [ ... ]"
    // The leaf id is the index within that vector.
    let mut in_nodes = false;
    let mut current_id: usize = 0;

    for line in dbg.lines() {
        let t = line.trim();

        // Find the start of the `nodes: [` section.
        if t.starts_with("nodes:") && t.contains('[') {
            in_nodes = true;
            current_id = 0;
            continue;
        }

        if !in_nodes {
            continue;
        }

        // End of the nodes list.
        if t.starts_with(']') {
            break;
        }

        // Each node is emitted by Debug on its own line, like:
        // "Leaf { value: 12, parent: Some(3) },"
        // "Add { children: [..], parent: .. },"
        // "Dead,"
        let is_node_line = t.starts_with("Dead")
            || t.starts_with("Leaf {")
            || t.starts_with("Neg {")
            || t.starts_with("Add {")
            || t.starts_with("Mul {")
            || t.starts_with("Sub {")
            || t.starts_with("Div {")
            || t.starts_with("Pow {");

        if !is_node_line {
            continue;
        }

        if t.starts_with("Leaf {") {
            if let Some(v) = t
                .split("value:")
                .nth(1)
                .and_then(|s| s.split(',').next())
                .and_then(|s| s.trim().parse::<i128>().ok())
            {
                out.push(LeafChip {
                    id: current_id,
                    value: v,
                });
            }
        }

        current_id += 1;
    }

    out
}

