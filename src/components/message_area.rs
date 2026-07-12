// file: src/components/message_area.rs
use leptos::prelude::*;

use crate::state::AppState;

#[component]
pub fn MessageArea() -> impl IntoView {
    let state = AppState::use_state();

    view! {
        <div class="message-area">
            {move || {
                if let Some(err) = state.error_msg.get() {
                    view! { <div class="error-message">{err}</div> }.into_any()
                } else if let Some(succ) = state.success_msg.get() {
                    view! { <div class="success-message">{succ}</div> }.into_any()
                } else {
                    view! {
                        <div class="info-message">
                            "Click on leaf numbers to select two. Then solve."
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
