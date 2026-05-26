use leptos::prelude::*;

use crate::user_algebra_bridge::{
    leaf_chips_from_arena, parse_expression, solve_pair, LeafChip, SolveOutcome,
};
use crate::ui::expression::{render_infix_expression, OnLeafClick};
use leptos::prelude::IntoAny;

/// Root application component.
///
/// The app flow is:
/// 1) user types an expression and clicks "Load"
/// 2) UI shows expression + list of leaf "number chips" (id/value)
/// 3) user selects two ids and clicks "Solve"
/// 4) we call into `user_algebra` to validate/apply the move; if valid we update the arena + steps
#[leptos::component]
pub fn App() -> impl IntoView {
    // --- UI state ---------------------------------------------------------

    // Raw text expression input typed by the user.
    let input = RwSignal::new(String::from("2+3*4"));

    // The current arena (game state) produced by `user_algebra`.
    let arena = RwSignal::new(None::<user_algebra::Arena>);

    // Derived list of leaf nodes (numbers) shown as selectable chips.
    //
    // NOTE: this is currently extracted by parsing `Debug` output from the arena.
    let leaf_chips = RwSignal::new(Vec::<LeafChip>::new());

    // Selected leaf ids for the next move.
    let selected_a = RwSignal::new(None::<usize>);
    let selected_b = RwSignal::new(None::<usize>);

    // Step history shown on the right panel.
    //
    // Each entry is currently a simple "before → after" string.
    let steps = RwSignal::new(Vec::<String>::new());

    // Status line (success/error) shown under the expression.
    let status = RwSignal::new(None::<String>);
    let status_is_error = RwSignal::new(false);

    // --- Actions ----------------------------------------------------------

    // Parse the input expression into a fresh arena and reset the game UI state.
    let rebuild = move || {
        let s = input.get_untracked();

        // `user_algebra` currently panics on parse errors (it unwraps internally),
        // so we keep the UI simple and assume input is valid for now.
        let a = parse_expression(s);

        // Reset all derived/selection state.
        steps.update(|v| v.clear());
        status.set(None);
        status_is_error.set(false);
        selected_a.set(None);
        selected_b.set(None);

        leaf_chips.set(leaf_chips_from_arena(&a));
        arena.set(Some(a));
    };

    // Selection logic for clickable leaves inside the infix expression renderer.
    //
    // Rules:
    // - click toggles a selection on/off
    // - at most two selections at any time
    // - when two are selected, clicking a third leaf does nothing (user must unselect one)
    let on_leaf_click: OnLeafClick = std::rc::Rc::new(move |id: usize| {
        // Toggle off if clicked leaf is already selected.
        if selected_a.get_untracked() == Some(id) {
            selected_a.set(None);
            status.set(None);
            status_is_error.set(false);
            return;
        }
        if selected_b.get_untracked() == Some(id) {
            selected_b.set(None);
            status.set(None);
            status_is_error.set(false);
            return;
        }

        // Fill selection slots A then B.
        if selected_a.get_untracked().is_none() {
            selected_a.set(Some(id));
            status.set(None);
            status_is_error.set(false);
            return;
        }
        if selected_b.get_untracked().is_none() {
            selected_b.set(Some(id));
            status.set(None);
            status_is_error.set(false);
            return;
        }

        // Both filled -> do nothing; user must unselect one.
        status.set(Some("Only two numbers can be selected. Click one again to unselect.".into()));
        status_is_error.set(true);
    });

    // Apply the move: solve two selected leaves by id.
    //
    // The logic crate decides whether the move is valid:
    // - `Ok`: arena is updated (numbers reduced)
    // - `Err`: arena stays unchanged, we show an error message
    let try_solve = move || {
        let Some(id1) = selected_a.get_untracked() else {
            status.set(Some("Pick 2 numbers to solve.".to_string()));
            status_is_error.set(true);
            return;
        };
        let Some(id2) = selected_b.get_untracked() else {
            status.set(Some("Pick 2 numbers to solve.".to_string()));
            status_is_error.set(true);
            return;
        };

        arena.update(|maybe| {
            let Some(ar) = maybe.as_mut() else {
                status.set(Some("Load an expression first.".to_string()));
                status_is_error.set(true);
                return;
            };

            // Capture a human-readable before/after for the "Steps" timeline.
            let before = ar.print_infix();
            match solve_pair(ar, id1, id2) {
                SolveOutcome::Applied(ok) => {
                    let after = ar.print_infix();
                    steps.update(|v| v.push(format!("{before}  →  {after} ({ok:?})")));

                    // Refresh selection/leaf list after a successful reduction.
                    leaf_chips.set(leaf_chips_from_arena(ar));
                    selected_a.set(None);
                    selected_b.set(None);

                    status.set(Some("OK".to_string()));
                    status_is_error.set(false);
                }
                SolveOutcome::Rejected(err) => {
                    status.set(Some(format!("Error: {err:?}")));
                    status_is_error.set(true);
                }
            }
        });
    };

    let on_solve_clicked = move |_| try_solve();

    // Enter key shortcut: if two leaves are selected, hitting Enter triggers Solve.
    let on_input_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() != "Enter" {
            return;
        }
        ev.prevent_default();
        try_solve();
    };

    // --- View -------------------------------------------------------------

    view! {
        <div class="app">
            <section class="card">
                // Left panel header.
                <h1 class="title">"Expression"</h1>

                // Expression input + Load button.
                <div class="row">
                    <input
                        class="input"
                        prop:value=move || input.get()
                        on:input=move |ev| input.set(event_target_value(&ev))
                        on:keydown=on_input_keydown
                        placeholder="e.g. 2+3*4"
                    />
                    <button class="btn primary" on:click=move |_| rebuild()>
                        "Load"
                    </button>
                </div>

                <div style="height: 10px"></div>

                // Selection row: shows which two ids are currently selected and lets user solve/clear.
                <div class="row">
                    <span class="chip">
                        "Pick A: "
                        {move || selected_a.get().map(|v| v.to_string()).unwrap_or_else(|| "none".into())}
                    </span>
                    <span class="chip">
                        "Pick B: "
                        {move || selected_b.get().map(|v| v.to_string()).unwrap_or_else(|| "none".into())}
                    </span>

                    <button class="btn primary" on:click=on_solve_clicked>
                        "Solve"
                    </button>

                    <button class="btn" on:click=move |_| {
                        selected_a.set(None);
                        selected_b.set(None);
                        status.set(None);
                        status_is_error.set(false);
                    }>
                        "Clear picks"
                    </button>
                </div>

                <div style="height: 10px"></div>

                // The current arena rendered as an infix string.
                <div class="expr">
                    {move || arena.with(|maybe| {
                        let Some(a) = maybe.as_ref() else {
                            return view! { <span>"Load an expression to start."</span> }.into_any();
                        };
                        render_infix_expression(
                            a,
                            selected_a,
                            selected_b,
                            on_leaf_click.clone(),
                        )
                    })}
                </div>

                <div style="height: 10px"></div>

                // Secondary view: list of leaf ids for debugging/learning.
                // This is useful because the logic crate operates on leaf ids.
                <div class="row">
                    <span class="chip">"Leaf ids:"</span>
                    <For
                        each=move || leaf_chips.get()
                        key=|c| c.id
                        children=move |c| {
                            view! {
                                <span class="chip">{format!("#{}={}", c.id, c.value)}</span>
                            }
                        }
                    />
                </div>

                <div style="height: 10px"></div>

                // Status line (errors show in red, success in green).
                <div class="row">
                    <span class="chip">
                        <span class=move || if status_is_error.get() { "status bad" } else { "status good" }>
                            "Status:"
                        </span>
                        {move || status.get().unwrap_or_else(|| "—".into())}
                    </span>
                </div>
            </section>

            // Right panel: step timeline.
            <aside class="card">
                <h2 class="title">"Steps"</h2>
                <div class="steps">
                    <For
                        each=move || steps.get()
                        key=|s| s.clone()
                        children=move |s| view! {
                            <div class="step">
                                <div class="k">"Rewrite"</div>
                                <div class="v">{s}</div>
                            </div>
                        }
                    />
                </div>
            </aside>
        </div>
    }
}
