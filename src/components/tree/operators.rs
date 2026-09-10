// file: src/components/tree/operators.rs
use leptos::prelude::*;
use user_algebra::arena::{Arena, Id};

use super::tree_node::TreeNode;

/// Render per operatori n-ari (Add / Mul), con simbolo ripetuto tra i figli.
pub fn render_nary_op(symbol: &'static str, children: Vec<Id>, arena: Arena) -> impl IntoView {
    let mut child_views = Vec::new();
    let last_idx = children.len().saturating_sub(1);

    for (idx, child_id) in children.into_iter().enumerate() {
        child_views.push(view! { <TreeNode arena={arena.clone()} id={child_id} /> }.into_any());
        if idx < last_idx {
            child_views.push(view! { <span class="op-symbol-inline">{symbol}</span> }.into_any());
        }
    }

    view! { <div class="op-node nary">{child_views}</div> }
}

/// Render per operatori binari (Sub / Div / Pow).
pub fn render_binary_op(symbol: &'static str, left: Id, right: Id, arena: Arena) -> impl IntoView {
    view! {
        <div class="op-node binary">
            <TreeNode arena={arena.clone()} id={left} />
            <span class="op-symbol">{symbol}</span>
            <TreeNode arena={arena} id={right} />
        </div>
    }
}
