# Axiom Programming Language

Axiom is a minimal, statically-typed interpreted programming language implemented in Rust. It is an educational project that demonstrates how a full language pipeline is built, from lexical analysis through to interpretation.

✨ Overview

Axiom is small but well-structured. The project implements a complete compilation-style pipeline that runs entirely in-process:

Source Code
  ↓
Lexer → Parser → AST → Type Checker → Interpreter
  ↓
Result

The design goals are clarity, a strong separation of compiler stages, and an emphasis on static type checking before execution. Axiom is ideal for learning Rust, compiler design, and interpreter implementation.

🚀 Features

- Variable bindings (`let`)
- Integer arithmetic with correct operator precedence
- A special `now` primitive that returns an incrementing global time value
- Static type checking (prevents invalid operations before execution)

Example language snippets:

```
let x = 5
x

let t = now
t
```

The `now` primitive advances a global time counter on each call (first call returns `Time(0)`).

🧠 Language Semantics

Types

Currently supported types:

- `Int`
- `Time`

Expressions

- Number literals
- Variable references
- Binary arithmetic (`+`, `-`, `*`, `/`) with precedence
- `now` primitive

Statements

- `let` bindings
- Expression statements

🏗 Project Architecture

The source is organized to reflect distinct compilation stages:

- `src/main.rs` — entry point that wiresthe pipeline
- `src/lexer.rs` — tokenization
- `src/parser.rs` — recursive-descent parser
- `src/ast.rs` — AST definitions
- `src/typechecker.rs` — static type checking pass
- `src/types.rs` — type definitions
- `src/interpreter.rs` — AST evaluation / runtime
- `src/diagnostic.rs` — error reporting

🔄 Compilation Pipeline

1. Lexer — converts raw characters into tokens (e.g. `Let`, `Ident("x")`, `Number(5)`, `EOF`).
2. Parser — builds an AST using recursive-descent parsing with precedence handling.
3. Type Checker — verifies types and prevents invalid operations before execution.
4. Interpreter — evaluates the checked AST, manages the environment and the global time counter.

🛠 How to Run

Requirements

- Rust (stable toolchain)
- Cargo

Run the project from the repository root:

```bash
cargo run
```

The test program is currently defined in `src/main.rs`; edit it to try different inputs.

📌 Example Program

Put this into `src/main.rs` or the current input program:

```
let t = now
t
```

Expected output (example):

```
Result: Some(Time(0))
```

🔮 Roadmap

Planned improvements:

- Block scopes and scoped variables
- Undefined variable diagnostics
- Boolean type and comparison operators
- Function definitions and calls
- A REPL (Read–Eval–Print Loop)
- Improved error messages and diagnostics
- Optional bytecode backend

🤝 Contributing

Contributions, experiments, and ideas are welcome. Suggested areas to explore:

- New operators and types
- Standard library primitives
- Better diagnostics and error localization
- Optimizations and small transformations

When contributing, follow idiomatic Rust and keep changes narrowly focused.

📜 License

Choose a license for the repository. For an educational project, the MIT License is a common choice.

---

Why this project exists

Axiom exists to answer the question: "What actually happens when you write code?" Instead of using an existing compiler, we implement the full pipeline to learn how languages work under the hood.

Happy hacking — explore the pipeline and extend Axiom!
