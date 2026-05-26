//! Infix expression renderer with clickable leaf nodes.
//!
//! We traverse `user_algebra::Arena` starting from `Arena::root()` and render
//! operators as plain text while rendering leaf nodes (`Node::Leaf`) as buttons.
//! The UI owns selection state; this module only calls back with the clicked id.

use leptos::prelude::*;
use leptos::prelude::{AnyView, IntoAny};

/// Shared callback signature for leaf clicks.
pub type OnLeafClick = std::rc::Rc<dyn Fn(usize)>;

/// Render the arena as infix notation where leaves are clickable.
pub fn render_infix_expression(
    arena: &user_algebra::Arena,
    selected_a: RwSignal<Option<usize>>,
    selected_b: RwSignal<Option<usize>>,
    on_leaf_click: OnLeafClick,
) -> AnyView {
    render_node(arena, arena.root(), selected_a, selected_b, on_leaf_click)
}

fn render_node(
    arena: &user_algebra::Arena,
    id: usize,
    selected_a: RwSignal<Option<usize>>,
    selected_b: RwSignal<Option<usize>>,
    on_leaf_click: OnLeafClick,
) -> AnyView {
    use user_algebra::Node;

    match arena.get(id) {
        Node::Dead => view! { <span></span> }.into_any(),

        Node::Leaf { value, .. } => {
            // Leaf nodes are the interactive pieces in the game.
            // We highlight them when selected.
            let is_selected = move || {
                selected_a.get() == Some(id) || selected_b.get() == Some(id)
            };

            let on_click = {
                let on_leaf_click = on_leaf_click.clone();
                move |_| (on_leaf_click)(id)
            };

            view! {
                <button
                    class=move || {
                        if is_selected() { "leaf-btn leaf-btn--selected" } else { "leaf-btn" }
                    }
                    on:click=on_click
                    type="button"
                    title=format!("leaf id #{id}")
                >
                    {value.to_string()}
                </button>
            }
            .into_any()
        }

        Node::Neg { child, .. } => view! {
            <span class="op">"-"</span>
            {render_node(arena, *child, selected_a, selected_b, on_leaf_click)}
        }
        .into_any(),

        Node::Add { children, .. } => join_children(
            arena,
            children,
            " + ",
            selected_a,
            selected_b,
            on_leaf_click,
        ),

        Node::Mul { children, .. } => join_children(
            arena,
            children,
            " * ",
            selected_a,
            selected_b,
            on_leaf_click,
        ),

        Node::Sub { left, right, .. } => binary(
            arena,
            *left,
            " - ",
            *right,
            selected_a,
            selected_b,
            on_leaf_click,
        ),

        Node::Div { left, right, .. } => binary(
            arena,
            *left,
            " / ",
            *right,
            selected_a,
            selected_b,
            on_leaf_click,
        ),

        Node::Pow { left, right, .. } => binary(
            arena,
            *left,
            " ^ ",
            *right,
            selected_a,
            selected_b,
            on_leaf_click,
        ),
    }
}

fn join_children(
    arena: &user_algebra::Arena,
    children: &[usize],
    sep: &'static str,
    selected_a: RwSignal<Option<usize>>,
    selected_b: RwSignal<Option<usize>>,
    on_leaf_click: OnLeafClick,
) -> AnyView {
    let mut out: Vec<AnyView> = Vec::new();

    for (i, &child) in children.iter().enumerate() {
        if i != 0 {
            out.push(view! { <span class="op">{sep}</span> }.into_any());
        }
        out.push(render_node(
            arena,
            child,
            selected_a,
            selected_b,
            on_leaf_click.clone(),
        ));
    }

    view! { <span class="expr-inline">{out}</span> }.into_any()
}

fn binary(
    arena: &user_algebra::Arena,
    left: usize,
    op: &'static str,
    right: usize,
    selected_a: RwSignal<Option<usize>>,
    selected_b: RwSignal<Option<usize>>,
    on_leaf_click: OnLeafClick,
) -> AnyView {
    view! {
        <span class="expr-inline">
            {render_node(arena, left, selected_a, selected_b, on_leaf_click.clone())}
            <span class="op">{op}</span>
            {render_node(arena, right, selected_a, selected_b, on_leaf_click)}
        </span>
    }
    .into_any()
}
