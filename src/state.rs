// file: src/state.rs
use leptos::prelude::*;
use user_algebra::{messages::{SolveError, SolveOk}, *};

/// Stato applicativo condiviso. E' `Copy` (tutti i campi sono signal, che sono
/// handle leggeri) quindi puo' essere passato per valore alle closure senza
/// dover clonare nulla.
#[derive(Clone, Copy)]
pub struct AppState {
    pub arena: ReadSignal<Option<Arena>>,
    set_arena: WriteSignal<Option<Arena>>,

    pub selected_leaves: ReadSignal<Vec<Id>>,
    set_selected_leaves: WriteSignal<Vec<Id>>,

    pub error_msg: ReadSignal<Option<String>>,
    set_error_msg: WriteSignal<Option<String>>,

    pub success_msg: ReadSignal<Option<String>>,
    set_success_msg: WriteSignal<Option<String>>,

    pub input_text: ReadSignal<String>,
    pub set_input_text: WriteSignal<String>,
}


impl AppState {
    fn new(initial_expr: &str) -> Self {
        let (arena, set_arena) = signal(None);
        let (selected_leaves, set_selected_leaves) = signal(Vec::<Id>::new());
        let (error_msg, set_error_msg) = signal(None::<String>);
        let (success_msg, set_success_msg) = signal(None::<String>);
        let (input_text, set_input_text) = signal(initial_expr.to_string());

        Self {
            arena,
            set_arena,
            selected_leaves,
            set_selected_leaves,
            error_msg,
            set_error_msg,
            success_msg,
            set_success_msg,
            input_text,
            set_input_text,
        }
    }

    /// Crea lo stato e lo registra nel context. Va chiamato una sola volta,
    /// nel componente root (`App`).
    pub fn provide(initial_expr: &str) -> Self {
        let state = Self::new(initial_expr);
        provide_context(state);
        state
    }

    /// Recupera lo stato da qualsiasi componente figlio di `App`.
    pub fn use_state() -> Self {
        expect_context::<AppState>()
    }

    pub fn parse_expression(&self) {
        let expr_str = self.input_text.get();
        self.set_error_msg.set(None);
        self.set_success_msg.set(None);
        match Arena::from_str(expr_str) {
            Ok(new_arena) => {
                self.set_arena.set(Some(new_arena));
                self.set_selected_leaves.set(Vec::new());
            }
            Err(err) => self.set_error_msg.set(Some(format!("Parse error: {}", err))),
        }
    }

    pub fn solve_selected(&self) {
        let selected = self.selected_leaves.get();
        if selected.len() != 2 {
            self.set_error_msg
                .set(Some("Select exactly two leaves".to_string()));
            return;
        }
        let id1 = selected[0];
        let id2 = selected[1];
        self.set_error_msg.set(None);
        self.set_success_msg.set(None);

        let mut outcome = None;
        self.set_arena.update(|arena| {
            outcome = Some(arena.as_mut().expect("arena is none").solve_leaves(id1, id2));
        });

        match outcome.unwrap() {
            Ok(solve_ok) => {
                let result = match solve_ok {
                    SolveOk::Reduced { result, .. } => result,
                };
                self.set_success_msg.set(Some(format!("Result = {}", result)));
                self.set_selected_leaves.set(Vec::new());
            }
            Err(e) => {
                let msg = match e {
                    SolveError::InvalidId => "Invalid leaf ID",
                    SolveError::NotALeaf => "Not a leaf",
                    SolveError::SameId => "Same leaf twice",
                    SolveError::NoSharedParent => "No shared parent",
                    SolveError::ParentNotOperable => "Cannot combine",
                    SolveError::DivisionByZero=>"Cannot divide by zero",
                    SolveError::InvalidExponent=>"Invalid exponent value",
                    SolveError::NotExact=>"Division is not exact",
                    SolveError::Overflow=>"Numbers to high"
                };
                self.set_error_msg.set(Some(msg.to_string()));
            }
        }
    }

    pub fn clear_selection(&self) {
        self.set_selected_leaves.set(Vec::new());
    }

    pub fn toggle_leaf(&self, leaf_id: Id) {
        self.set_selected_leaves.update(|selected| {
            if selected.contains(&leaf_id) {
                selected.retain(|&id| id != leaf_id);
            } else if selected.len() < 2 {
                selected.push(leaf_id);
            } else {
                selected[1] = leaf_id;
            }
        });
    }
}
