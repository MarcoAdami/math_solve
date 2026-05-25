//! App entrypoint.
//!
//! This crate is a Leptos CSR (client-side rendering) app built for Trunk.
//! `index.html` contains `<main id="app"></main>` and Trunk injects the WASM bundle.

mod ui;
mod user_algebra_bridge;

fn main() {
    // Make panics show up in the browser console with a useful stack trace.
    console_error_panic_hook::set_once();

    // Mount our root Leptos component into the document body.
    leptos::mount::mount_to_body(ui::App);
}

