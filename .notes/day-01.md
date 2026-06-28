# Questions

## 1. Are macros similar to decorators in TypeScript?

No its not the same, macros expand into rust code.

## 2. What does the `expect` method do?

When we have a `Result` enum, `Ok` and `Err` are its values.

`.expect("Failed to read line")`:

- if `Result` is `Ok`, return the value
- if `Result` is `Err`, crash the program and print the `Failed to read line` error message

## 3. What are crates, traits, functions, and methods?

- **`crates`** — package similar to `npm`
- **`traits`** - TODO: need to understand this
- **`method`** — called on a value

The below is a **function**:

```rust
fn main() {
  println!("hello")
}
```

Let's say we have a value like this:

```rust
let mut guess = String::new();
guess.trim()
```

`guess.trim()` is a **method**.

## 4. What is `match`? Is there if/else? Why not if/else here?

```rust
match guess.cmp(&secret_number) {
  Ordering::Less => println!("Too small!"),
  Ordering::Greater => println!("Too big!"),
  Ordering::Equal => {
    println!("You win!");
    break;
  }
}
```

Since the `guess.cmp` returns an `Operator` `enum`, `match` seems right fit as how we will `switch` in other languages
