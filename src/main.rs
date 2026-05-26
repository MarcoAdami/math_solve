// file: src/main.rs
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::IntoView;
use leptos::prelude::OnAttribute;
use leptos::prelude::PropAttribute;
use leptos::prelude::event_target_value;
use leptos::{
    mount::mount_to_body,
    reactive::{
        signal::{ReadSignal, signal},
        traits::{Get, Set, Update},
    },
    tachys::view::any_view::IntoAny,
    *,
};
use user_algebra::history::History;
use user_algebra::history::arena::Arena;
use user_algebra::history::arena::Id;
use user_algebra::history::arena::node::Node;
use user_algebra::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    // Replace the signal declarations
    let (history, set_history) =
        signal(History::from_str("(2 + 3) * 4".to_string()).unwrap_or_else(|_| History::new()));
    
    let (selected_leaves, set_selected_leaves) = signal(Vec::<Id>::new());
    let (error_msg, set_error_msg) = signal(None::<String>);
    let (success_msg, set_success_msg) = signal(None::<String>);
    let (input_text, set_input_text) = signal("(2 + 3) * 4".to_string());

    // Parse expression: rebuild history from input
    let parse_expression = move |_| {
        let expr_str = input_text.get();
        set_error_msg.set(None);
        set_success_msg.set(None);
        match History::from_str(expr_str) {
            Ok(new_history) => {
                set_history.set(new_history);
                set_selected_leaves.set(Vec::new());
            }
            Err(err) => set_error_msg.set(Some(format!("Parse error: {}", err))),
        }
    };

    // Solve: use history.solve_last
    let solve_selected = move |_| {
        

        let selected = selected_leaves.get();
        if selected.len() != 2 {
            set_error_msg.set(Some("Select exactly two leaves".to_string()));
            return;
        }
        let id1 = selected[0];
        let id2 = selected[1];
        set_error_msg.set(None);
        set_success_msg.set(None);
        let mut outcome = None;
        set_history.update(|hist| {
            outcome = Some(hist.solve_last(id1, id2));
        });

        // Inside solve_selected, before calling solve_history
        let hist = history.get();
        let arena = hist.get_last();
        let p1 = arena.get(id1).parent();
        let p2 = arena.get(id2).parent();
        leptos::logging::log!("Leaf {} parent: {:?}, leaf {} parent: {:?}", id1, p1, id2, p2);
        
        match outcome.unwrap() {
            Ok(solve_ok) => {
                let result = match solve_ok {
                    SolveOk::Reduced { result, .. } => result,
                };
                set_success_msg.set(Some(format!("Result = {}", result)));
                set_selected_leaves.set(Vec::new());
            }
            Err(e) => {
                let msg = match e {
                    SolveError::InvalidId => "Invalid leaf ID",
                    SolveError::NotALeaf => "Not a leaf",
                    SolveError::SameId => "Same leaf twice",
                    SolveError::NoSharedParent => "No shared parent",
                    SolveError::ParentNotOperable => "Cannot combine",
                };
                set_error_msg.set(Some(msg.to_string()));
            }
        }
    };

    let clear_selection = move |_| {
        set_selected_leaves.set(Vec::new());
    };

    let on_leaf_click = move |leaf_id: Id| {
        set_selected_leaves.update(|selected| {
            if selected.contains(&leaf_id) {
                selected.retain(|&id| id != leaf_id);
            } else if selected.len() < 2 {
                selected.push(leaf_id);
            } else {
                // Replace the second selected leaf with the new one
                selected[1] = leaf_id;
            }
        });
    };

    view! {
        <div class="app-container">
            <h1> Expression Tree Visualizer</h1>
            <div class="input-panel">
                <input
                    type="text"
                    class="expression-input"
                    prop:value={input_text}
                    on:input={move |ev| set_input_text.set(event_target_value(&ev))}
                />
                <button on:click={parse_expression}> Parse & Build Tree</button>
                <button on:click={solve_selected}> Solve Selected Leaves</button>
                <button on:click={clear_selection}> Clear Selection</button>
            </div>

            <div class="message-area">
                {move || {
                    if let Some(err) = error_msg.get() {
                        view! { <div class="error-message"> {err}</div> }.into_any()
                    } else if let Some(succ) = success_msg.get() {
                        view! { <div class="success-message"> {succ}</div> }.into_any()
                    } else {
                        view!{<div class="info-message"> "Click on leaf numbers to select two. Then solve."</div>}.into_any()
                    }
                }}
            </div>

            <div class="tree-container">
                {move || {
                    let hist = history.get();
                    if hist.len() == 0 {
                        view! { <div class="empty-tree">No expression</div> }.into_any()
                    } else {
                        let arena = hist.get_last();
                        let root_id = arena.root();
                        view! { <TreeNode arena={arena.clone()} id={root_id} selected={selected_leaves} on_leaf_click={on_leaf_click} /> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

/// Recursive component that renders a single node and its children.
// #[component]
// fn TreeNode(
//     arena: ReadSignal<Arena>,
//     id: Id,
//     selected: ReadSignal<Vec<Id>>,
//     on_leaf_click: impl Fn(Id) + 'static + Clone,
// ) -> impl IntoView {
//     // Retrieve the node; if arena changed, reactivity will re-run this component.
//     let node = move || arena.get().get(id).clone();

//     let node_data = node();
//     match node_data {
//         Node::Dead => view! { <></> }.into_any(),
//         Node::Leaf { value, parent: _ } => {
//             let is_selected = move || selected.get().contains(&id);
//             let leaf_id = id;
//             let click_cb = on_leaf_click.clone();
//             view! {
//                 <div
//                     class="leaf-node"
//                     class:selected=is_selected
//                     on:click=move |_| click_cb(leaf_id)
//                 >
//                     {value}
//                 </div>
//             }
//             .into_any()
//         }
//         Node::Neg { child, parent: _ } => {
//             view! {
//                 <div class="op-node unary">
//                     <span class="op-symbol">-</span>
//                     <TreeNode arena={arena} id={child} selected={selected} on_leaf_click={on_leaf_click.clone()} />
//                 </div>
//             }
//             .into_any()
//         }
//         Node::Add { children, parent: _ } => {
//             render_nary_op("+", children, arena, selected, on_leaf_click).into_any()
//         }
//         Node::Mul { children, parent: _ } => {
//             render_nary_op("×", children, arena, selected, on_leaf_click).into_any()
//         }
//         Node::Sub { left, right, parent: _ } => {
//             render_binary_op("-", left, right, arena, selected, on_leaf_click).into_any()
//         }
//         Node::Div { left, right, parent: _ } => {
//             render_binary_op("÷", left, right, arena, selected, on_leaf_click).into_any()
//         }
//         Node::Pow { left, right, parent: _ } => {
//             render_binary_op("^", left, right, arena, selected, on_leaf_click).into_any()
//         }
//     }
// }

#[component]
fn TreeNode(arena: Arena, id: Id, selected: ReadSignal<Vec<Id>>, on_leaf_click: impl Fn(Id) + 'static + Clone) -> impl IntoView {
    let node = arena.get(id).clone();

    match node {
        Node::Dead => view! { <></> }.into_any(),
        Node::Leaf { value, .. } => {
            let is_selected = move || selected.get().contains(&id);
            let leaf_id = id;
            let click_cb = on_leaf_click.clone();
            view! {
                <div class="leaf-node" class:selected=is_selected on:click=move |_| click_cb(leaf_id)>
                    {value}
                </div>
            }.into_any()
        }
        Node::Neg { child, .. } => {
            view! {
                <div class="op-node unary">
                    <span class="op-symbol">-</span>
                    <TreeNode arena={arena} id={child} selected={selected} on_leaf_click={on_leaf_click.clone()} />
                </div>
            }.into_any()
        }
        Node::Add { children, .. } => {
            render_nary_op("+", children, arena, selected, on_leaf_click).into_any()
        }
        Node::Mul { children, parent: _ } => {
            render_nary_op("×", children, arena, selected, on_leaf_click).into_any()
        }
        Node::Sub { left, right, parent: _ } => {
            render_binary_op("-", left, right, arena, selected, on_leaf_click).into_any()
        }
        Node::Div { left, right, parent: _ } => {
            render_binary_op("÷", left, right, arena, selected, on_leaf_click).into_any()
        }
        Node::Pow { left, right, parent: _ } => {
            render_binary_op("^", left, right, arena, selected, on_leaf_click).into_any()
        }
    }
}

/// Helper to render n-ary operators (Add/Mul)
fn render_nary_op(
    symbol: &'static str,
    children: Vec<Id>,
    arena: Arena,
    selected: ReadSignal<Vec<Id>>,
    on_leaf_click: impl Fn(Id) + 'static + Clone,
) -> impl IntoView {
    let mut child_views = Vec::new();
    for (idx, child_id) in children.clone().into_iter().enumerate() {
        child_views.push(
            view! {
                <TreeNode arena={arena.clone()} id={child_id} selected={selected} on_leaf_click={on_leaf_click.clone()} />
            }
            .into_any(),
        );
        if idx < children.len() - 1 {
            child_views.push(view! { <span class="op-symbol-inline">{symbol}</span> }.into_any());
        }
    }
    view! { <div class="op-node nary">{child_views}</div> }.into_any()
}

/// Helper to render binary operators (Sub, Div, Pow)
fn render_binary_op(
    symbol: &'static str,
    left: Id,
    right: Id,
    arena: Arena,
    selected: ReadSignal<Vec<Id>>,
    on_leaf_click: impl Fn(Id) + 'static + Clone,
) -> impl IntoView {
    view! {
        <div class="op-node binary">
            <TreeNode arena={arena.clone()} id={left} selected={selected} on_leaf_click={on_leaf_click.clone()} />
            <span class="op-symbol">{symbol}</span>
            <TreeNode arena={arena} id={right} selected={selected} on_leaf_click={on_leaf_click} />
        </div>
    }
    .into_any()
}
