use leptos::prelude::*;

use crate::user_algebra_bridge::{
    leaf_chips_from_arena_debug, parse_expression, solve_pair, LeafChip, SolveOutcome,
};

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

        leaf_chips.set(leaf_chips_from_arena_debug(&a));
        arena.set(Some(a));
    };

    // Apply the move: solve two selected leaves by id.
    //
    // The logic crate decides whether the move is valid:
    // - `Ok`: arena is updated (numbers reduced)
    // - `Err`: arena stays unchanged, we show an error message
    let on_solve_clicked = move |_| {
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
                    leaf_chips.set(leaf_chips_from_arena_debug(ar));
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
                    {move || arena.get().as_ref().map(|a| a.print_infix()).unwrap_or_else(|| "Load an expression to start.".into())}
                </div>

                <div style="height: 10px"></div>

                // Clickable list of leaf nodes (numbers).
                // The player can choose any two leaves; `user_algebra` will accept/reject the move.
                <div class="row">
                    <span class="chip">"Numbers (click two):"</span>
                    <For
                        each=move || leaf_chips.get()
                        key=|c| c.id
                        children=move |c| {
                            // Each chip click fills slot A then slot B.
                            // If both are set, clicking resets A to the new id and clears B.
                            let on_pick = move |_| {
                                if selected_a.get_untracked().is_none() {
                                    selected_a.set(Some(c.id));
                                    status.set(None);
                                    status_is_error.set(false);
                                } else if selected_b.get_untracked().is_none() {
                                    if selected_a.get_untracked() == Some(c.id) {
                                        status.set(Some("Pick two different numbers.".into()));
                                        status_is_error.set(true);
                                    } else {
                                        selected_b.set(Some(c.id));
                                        status.set(None);
                                        status_is_error.set(false);
                                    }
                                } else {
                                    selected_a.set(Some(c.id));
                                    selected_b.set(None);
                                    status.set(None);
                                    status_is_error.set(false);
                                }
                            };

                            view! {
                                <button class="btn" on:click=on_pick>
                                    {format!("#{} = {}", c.id, c.value)}
                                </button>
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
