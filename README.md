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

---

## Axiom’s Philosophy

Axiom is an experimental programming language focused on clarity, correctness, and intentional design.

It is currently under active development and evolving feature by feature. Rather than rushing toward complexity, Axiom grows through carefully implemented architectural layers: lexical analysis, parsing, type checking, and interpretation.

Axiom is not built to compete with production languages. It is built to understand them.

### 🎯 Core Principles

1. Architecture First

Every feature in Axiom must pass through a complete language pipeline:

Lexer → Parser → AST → Type Checker → Interpreter

No shortcuts.
No hidden execution rules.
No bypassing the type system.

If a feature cannot be cleanly modeled across these stages, it does not belong in the language.

2. Semantics Before Syntax

Axiom prioritizes semantic correctness over syntactic convenience.

Programs must:

- Be type-safe
- Be predictable
- Have well-defined behavior

Even as a small language, Axiom enforces structure through a static type system.

3. Minimalism With Intent

Axiom deliberately begins small:

- Integers
- Time values
- Expressions
- Variable bindings

Each addition is designed to expand expressive power without compromising clarity.

The goal is not to add features quickly. The goal is to add features correctly.

4. Explicitness Over Magic

Axiom avoids hidden behaviors.

State (like `now`) is explicit.
Types are explicit.
Evaluation order is deterministic.

The language aims to be understandable by reading its interpreter.

5. Educational Transparency

Axiom is designed to be readable both as a language and as an implementation.

The source code demonstrates:

- How interpreters are structured
- How recursive descent parsers work
- How static type checking is implemented
- How environments manage scope and values

Axiom is as much about learning language design as it is about writing programs.

### 🚧 Development Status

Axiom is currently in early-stage development.

Implemented:

- Variable bindings
- Arithmetic expressions
- Static type checking
- Interpreter with state (`now` primitive)

Planned:

- Scoped blocks
- Boolean type
- Comparison operators
- Functions
- REPL
- Improved diagnostics
- Possible bytecode backend

The language is evolving incrementally, with architectural discipline guiding each feature.

### 🌱 Long-Term Vision

Axiom aims to grow into a small but principled language that explores:

- Deterministic evaluation
- Explicit state modeling
- Static guarantees
- Minimal but expressive syntax
- Clean separation of compilation phases

It may remain an educational language.
It may evolve into something more experimental.

What matters is that its growth remains intentional.

### Why This Works

Notice what this does:

- It admits it’s under development
- It avoids pretending to be production-ready
- It sounds deliberate, not accidental

It frames the project as a research/learning language and communicates seriousness without exaggeration.

This no longer sounds like: "I made a toy language for fun."

It sounds like: "I am intentionally exploring programming language design."

And that’s powerful.

---

Why this project exists

Axiom exists to answer the question: "What actually happens when you write code?" Instead of using an existing compiler, we implement the full pipeline to learn how languages work under the hood.
