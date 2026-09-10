// file: src/app.rs
use leptos::prelude::*;

use crate::components::input_panel::InputPanel;
use crate::components::message_area::MessageArea;
use crate::components::tree::tree_node::TreeNode;
use crate::state::AppState;

#[component]
pub fn App() -> impl IntoView {
    // Unico punto in cui lo stato viene creato e messo nel context.
    let state = AppState::provide("2 + 3 * 4");

    view! {
        <div class="app-container">
            <h1>Move2Solve</h1>

            <InputPanel />
            <MessageArea />

            <div class="user-expression-container">
                {move || {
                    let arena = state.arena.get();
                    match arena{
                        None=>view! { <div class="empty-tree">No expression</div> }.into_any(),
                        Some(arena)=>{
                            let root_id = arena.root();
                            view! { <TreeNode arena={arena.clone()} id={root_id} /> }.into_any()
                        }
                    }
                }}
            </div>

            <div class="tree-expression-debug"></div>
        </div>
    }
}
