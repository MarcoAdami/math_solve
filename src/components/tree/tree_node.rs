// file: src/components/tree/tree_node.rs
use leptos::prelude::*;
use user_algebra::history::arena::node::Node;
use user_algebra::history::arena::{Arena, Id};

use super::leaf::Leaf;
use super::operators::{render_binary_op, render_nary_op};

/// Nodo ricorsivo dell'albero. Non tocca lo stato di selezione: quello
/// vive solo in `Leaf`, recuperato via context.
#[component]
pub fn TreeNode(arena: Arena, id: Id) -> impl IntoView {
    let node = arena.get(id).clone();

    match node {
        Node::Dead => view! { <></> }.into_any(),

        Node::Leaf { value, .. } => view! { <Leaf id={id} value={value} /> }.into_any(),

        Node::Neg { child, .. } => view! {
            <div class="op-node unary">
                <span class="op-symbol">-</span>
                <TreeNode arena={arena} id={child} />
            </div>
        }
        .into_any(),

        Node::Add { children, .. } => render_nary_op("+", children, arena).into_any(),
        Node::Mul { children, .. } => render_nary_op("×", children, arena).into_any(),

        Node::Sub { left, right, .. } => render_binary_op("-", left, right, arena).into_any(),
        Node::Div { left, right, .. } => render_binary_op("÷", left, right, arena).into_any(),
        Node::Pow { left, right, .. } => render_binary_op("^", left, right, arena).into_any(),
    }
}
