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
pub fn leaf_chips_from_arena(arena: &user_algebra::Arena) -> Vec<LeafChip> {
    // With `Arena::len()` exposed, we can enumerate ids and query each node directly.
    let mut out = Vec::new();
    for id in 0..arena.len() {
        match arena.get(id) {
            user_algebra::Node::Leaf { value, .. } => out.push(LeafChip { id, value: *value }),
            _ => {}
        }
    }
    out
}
