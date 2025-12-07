# Intro

When writing Rust code that contains `.expect()` calls, we need to provide a message that explains
why the panic is impossible here, e.g.:

```rust
fn main() {
    let items = make_items();
    let first = items.get(0).expect("`items` is initialized to 3 items");
}

fn make_items() -> Vec<i32> {
    vec![1, 2, 3]
}
```

The use of string message for `.expect()` is not ideal, as its contents is not checked by the compiler,
and it is hard to provide a compact message with a comprehensive explanation.

We can make some important observations:
- the `.expect()` call is actually a use of some invariant
- some other place in code must ensure that this invariant holds.

E.g., for the example code:
- the invariant is "`make_items` returns a non-empty vector"
- this invariant is ensured in the body of `make_items` function
- this invariant is used in the body of `main` function

This library aims to improve this by providing at least some of degree of compiler checking for `.expect()` calls.
We let the Rust's type system to check that all used references are proven in some other place of code.

With the help of `invariant_reference`, the example code is written as:
```rust
fn main() {
    let items = make_items();
    let first = items.get(0).unwrap_under_invariant::<MakeItemsReturnsNonEmpty>();
}

#[derive(Invariant)]
#[invariant(message = "make_items() returns non-empty vector")]
struct MakeItemsReturnsNonEmpty;

fn make_items() -> Vec<i32> {
    invariant_established!(MakeItemsReturnsNonEmpty);
    vec![1, 2, 3]
}
```

This style of code has several advantages:
- the invariants are explicitly defined in code
- the `invariant_established!` is placed close to the code that ensures that the invariant holds;
  during code review it is easier to spot mistakes in such code
- we can use "Find All References" function to find all places that depend on an invariant

# How to use

1. Add references to `invariant_reference` and `invariant_reference_derive` crates
2. Define the invariant as a struct with the `#[derive(Invariant)]` macro:
   ```
   #[derive(Invariant)]
   #[invariant(message = "the xyz vector is not empty", num_proofs = 1)]
   struct XyzVecIsNotEmpty;
   ```
   - provide the `message` that explains the essence of the invariant, i.e. what condition must hold true
   - optionally, if an invariant is established in multiple places (e.g., a struct has multiple constructors),
     specify the number of such code paths in the `num_proofs` attribute.
3. Invoke the `invariant_established!(XyzVecIsNotEmpty)` macro in the specific code place where it makes sense
   (e.g., in the body of a struct constructor)
   - use `invariant_established!(XyzVecIsNotEmpty[0])` macro if there are multiple such places
4. Invoke `OptionExt::unwrap_under_invariant::<XyzVecIsNotEmpty>()` or `ResultExt::unwrap_under_invariant::<XyzVecIsNotEmpty>()` instead of `.expect()`

# Notes and limitations

- The code will fail to compile if `invariant_established!` is not invoked (even if the invariant is not referenced) or invoked multiple times for the same invariant (unless using different indices)
- `invariant_established!` may only be called within the same crate, as this libraries relies on traits.
