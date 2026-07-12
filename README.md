# Move2Solve

## Why?
This project started from my experience as a math tutor. I saw a lot of my student struggling to understand the rules of algebra and I wondered for some time how to help the highschooler be more confident on this topic and my first idea was this. Create a environment where the algebraic expression are dynamic as they should be and not static symbol on a sheet of paper. 
I think this method could contribute to make students visualize in the right way expressions.

I am only at the start so let's see how it plays out.


## What it is?
This project is written in Leptos and Trunk. It is the visualizer of the real algebraic engine that is one of my private projects, `user_algebra`.


## How does it work?
It enables the user to insert the expression they want to solve and edit it following the rules of algebra.
Due to the fact that it is the visualizer of other private repos I will keep updated the list of allowed syntax and actions.

### Allowed syntax
- integeres up to 2^128-1 (u128)
- binary operators: `+ - * / ^`
- unary operators: `+ -`

### Allowed actions
- Solve operations in order of precedence


