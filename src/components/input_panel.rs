// file: src/components/input_panel.rs
use leptos::prelude::*;

use crate::state::AppState;

#[component]
pub fn InputPanel() -> impl IntoView {
    let state = AppState::use_state();

    view! {
        <div class="input-panel">
            <input
                type="text"
                class="expression-input"
                prop:value={state.input_text}
                on:input={move |ev| state.set_input_text.set(event_target_value(&ev))}
            />
            <button on:click={move |_| state.parse_expression()}>Parse & Build Tree</button>
            <button on:click={move |_| state.solve_selected()}>Solve Selected Leaves</button>
            <button on:click={move |_| state.clear_selection()}>Clear Selection</button>
        </div>
    }
}
