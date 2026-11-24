# Rust & Tokio Learning Notes & Cheat Sheet

A comprehensive reference covering the core concepts, syntax, and behaviors learned during the exploration of Rust fundamentals and Tokio's asynchronous runtime.

---

## Table of Contents
1. [Rust Compiler Architecture](#1-rust-compiler-architecture)
2. [Project Structure & Multiple Binaries](#2-project-structure--multiple-binaries)
3. [Rust Syntax & Statements](#3-rust-syntax--statements)
4. [Vectors & Type Inference](#4-vectors--type-inference)
5. [Printing & Formatting: `{}` vs `{:?}`](#5-printing--formatting--vs-)
6. [Functions (`fn`) vs Macros (`!`)](#6-functions-fn-vs-macros-)
7. [Async Rust & The Tokio Runtime](#7-async-rust--the-tokio-runtime)
8. [Complete Tokio Example & Patterns](#8-complete-tokio-example--patterns)

---

## 1. Rust Compiler Architecture

* **Self-Hosting:** The modern Rust compiler (`rustc`) is written in **Rust**, not C.
* **LLVM Backend:** Rust uses LLVM as its default backend for optimization and machine-code generation, which is written in **C++**.
* **Historical Context:** The very first prototype of Rust (2006–2010) was written in **OCaml**. Once Rust grew capable enough to compile itself, the OCaml compiler was retired.

---

## 2. Project Structure & Multiple Binaries

In Cargo, every source file must belong to a crate's module tree. Loose files placed directly under `src/` are ignored by `cargo` and `rust-analyzer` unless linked.

### How to manage extra executable files (e.g., experiments/labs):

#### Option A: The Idiomatic Convention (`src/bin/`)
Cargo automatically recognizes any `.rs` file inside `src/bin/` as a separate binary.
```text
my_project/
├── Cargo.toml
└── src/
    ├── main.rs               # cargo run
    └── bin/
        └── tokio-lab.rs      # cargo run --bin tokio-lab
```

#### Option B: Register in `Cargo.toml`
If the file stays directly in `src/`, explicitly register it:
```toml
[[bin]]
name = "tokio-lab"
path = "src/tokio-lab.rs"
```

---

## 3. Rust Syntax & Statements

### Range Expressions (`..` vs `..=`)
* **Exclusive Range (`start..end`):** `0..10` generates `0, 1, ..., 9` (10 items). `end` is **excluded**.
* **Inclusive Range (`start..=end`):** `0..=10` generates `0, 1, ..., 10` (11 items). `end` is **included**.
* **Underscores in numbers:** `100_000` is purely for visual readability. To the compiler, it is identical to `100000`.

### Statement Pitfall: "Path statement with no effect"
Writing an isolated variable expression ending with a semicolon does nothing:
```rust
i; // ⚠️ Warning: path statement with no effect (evaluates i and immediately discards it)
```

---

## 4. Vectors & Type Inference

```rust
let mut hs = vec![];
```

* **`Vec<T>`:** A resizable, growable array allocated on the **heap**.
* **`vec![]`:** A built-in macro. `vec![]` is equivalent to `Vec::new()`.
* **Bracket Convention:** Macros accept `()`, `[]`, or `{}`. `vec![]` uses square brackets by convention to mirror array literal syntax.
* **Type Inference:** Explicit annotations like `: Vec<i32>` are **optional**. When elements are pushed (e.g., `hs.push(...)`), Rust automatically infers the element type.

---

## 5. Printing & Formatting: `{}` vs `{:?}`

| Format Specifier | Trait | Intended Audience | Supported Types |
| :--- | :--- | :--- | :--- |
| **`{}`** | `Display` | End-users | Primitives (`i32`, `&str`, `bool`) |
| **`{:?}`** | `Debug` | Developers | Collections, structs, `JoinHandle`, internal types |

> [!NOTE]
> `JoinHandle` does **not** implement `Display`. To print it, use debug formatting (`{:?}`) or access its identifier via `h.id()`.

---

## 6. Functions (`fn`) vs Macros (`!`)

| Feature | Regular Function (`fn`) | Macro (`!`, `#[...]`) |
| :--- | :--- | :--- |
| **Execution Time** | **Runtime** | **Compile Time** |
| **Input** | Evaluated values & data | Source code tokens / AST |
| **Output** | A returned value | **Generated Rust code** (expands in place) |
| **Arguments** | Fixed count and types | Variable arguments (`println!`, `vec!`) |
| **Capability** | Normal computational logic | Metaprogramming & syntax transformation |

---

## 7. Async Rust & The Tokio Runtime

### The Division of Responsibilities
* **Standard Rust:** Provides only language keywords (`async`, `.await`) and the core `Future` trait. It includes **no built-in runtime engine**.
* **Tokio:** A 3rd-party asynchronous runtime crate providing the thread pool, work-stealing scheduler, I/O reactor, and timers.

### Core Tokio Primitives
* **`#[tokio::main]`:** An attribute macro that transforms an `async fn main()` into a standard synchronous `fn main()`, initializing the multi-threaded Tokio runtime engine under the hood.
* **`tokio::spawn(async move { ... })`:**
  * Spawns an async task concurrently onto Tokio's background thread pool.
  * **`move`** transfers ownership of captured variables (like loop index `i`) into the task closure so they remain valid when outer scopes finish.
* **`JoinHandle`:** The receipt/token returned by `tokio::spawn`.
  * `.await`: Pauses and waits for the task to finish, returning `Result<T, JoinError>`.
  * `.abort()`: Cancels the running task.
  * `.id()`: Returns a unique task identifier (`tokio::task::Id`).
* **`tokio::time::sleep`:** Requires a typed `std::time::Duration` (e.g., `Duration::from_secs(1)`), not a raw integer.

---

## 8. Complete Tokio Example & Patterns

```rust
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    // 1. Spawn concurrent tasks in the background
    for i in 0..5 {
        handles.push(tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Task {i} finished work");
            i * 10 // Return value
        }));
    }

    // 2. Await all handles before main exits
    for h in handles {
        // Inspect handle before .await consumes it
        println!("Awaiting task ID: {:?}", h.id());

        match h.await {
            Ok(result) => println!("Task returned: {result}"),
            Err(e) => eprintln!("Task panicked or cancelled: {e}"),
        }
    }

    println!("All background tasks completed!");
}
```

> [!IMPORTANT]
> If you don't await the handles (`for h in handles { h.await... }`), `main()` will terminate immediately upon finishing its synchronous loop, killing all background tasks before they have time to execute.
