# Math Solve Visualizer (Leptos CSR + Trunk)

This repo is a UI/visualizer for the `user_algebra` logic crate. It runs fully in the browser (WASM) using **Leptos (CSR)** and **Trunk**.

## Requirements

- Rust toolchain (edition 2024)
- `wasm32-unknown-unknown` target
- Trunk

## Setup

Add the WASM target:

```sh
rustup target add wasm32-unknown-unknown
```

Install Trunk (if you don't have it yet):

```sh
cargo install trunk
```

## Run (dev server)

From this folder:

```sh
trunk serve --open
```

Trunk will build the WASM bundle and open the app in your browser.

## Game / Moves

You start by loading an expression made only of:

- integers
- operators: `+ - * / ^`
- no parentheses

### Allowed move

1. Click **Load** to parse the expression into the game arena.
2. Under **Numbers (click two):** click two different number chips (each chip has an arena id like `#7 = 12`).
3. Click **Solve**.

What happens:

- The UI calls `user_algebra::solve_leaves_api(&mut arena, id1, id2)`.
- If the move is **valid**, the expression is updated and a new entry is appended to **Steps** (a `before → after` rewrite).
- If the move is **invalid**, the expression is unchanged and you get an error (e.g. the two leaves don’t share a parent operator).

### Notes

- The operator used is determined by the shared parent node in the arena (you do not choose an operator in the UI).
- Division and exponent follow the logic crate behavior (integer math).

