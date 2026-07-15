// file: src/components/input_panel.rs
use leptos::{ev::KeyboardEvent, prelude::*};

use crate::state::AppState;

#[component]
pub fn InputPanel() -> impl IntoView {
    let state = AppState::use_state();

    let handle_key = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            // Your logic here
            state.parse_expression()
            
        }
    };

    view! {
        <div class="input-panel">
            <input
                type="text"
                class="expression-input"
                prop:value={state.input_text}
                on:input={move |ev| state.set_input_text.set(event_target_value(&ev))}
                on:keydown=handle_key
            />
            <button on:click={move |_| state.parse_expression()}>Parse & Build Tree</button>
            <button on:click={move |_| state.solve_selected()}>Solve Selected Leaves</button>
            <button on:click={move |_| state.clear_selection()}>Clear Selection</button>
        </div>
    }
}
