// file: src/components/tree/leaf.rs
use leptos::prelude::*;
use user_algebra::arena::Id;

use crate::state::AppState;

// NOTA: adatta il tipo di `value` a quello reale del campo
// `Node::Leaf { value, .. }` (es. u128, o un tuo tipo numerico custom).
#[component]
pub fn Leaf(id: Id, value: i128) -> impl IntoView {
    let state = AppState::use_state();
    let is_selected = move || state.selected_leaves.get().contains(&id);

    view! {
        <div
            class="leaf-node"
            class:selected=is_selected
            on:click=move |_| state.toggle_leaf(id)
        >
            {value}
        </div>
    }
}
