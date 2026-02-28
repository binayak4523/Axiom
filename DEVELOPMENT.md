# Axiom Language - Development Guide

Welcome! This guide explains how the Axiom language interpreter works. Since you're new to Rust, I'll explain both the Rust concepts AND the interpreter architecture.

## Table of Contents
1. [Project Overview](#project-overview)
2. [Rust Basics You Need to Know](#rust-basics-you-need-to-know)
3. [Project Structure](#project-structure)
4. [The Compilation Pipeline](#the-compilation-pipeline)
5. [Detailed Component Breakdown](#detailed-component-breakdown)
6. [How to Run & Develop](#how-to-run--develop)

---

## Project Overview

**Axiom** is a simple programming language interpreter written in Rust. It's a "toy language" designed to help you learn how programming languages work under the hood.

### What Axiom Can Do
- **Variable assignment**: `let x = 5`
- **Arithmetic**: `2 + 3 * 4`
- **Time tracking**: `now` - a special value that returns an incrementing time counter
- **Expressions**: `let t = now` then `t` to use the variable

### Example Program
```
let t = now
t
```
This creates a variable `t` that holds the current "time", then returns it.

---

## Rust Basics You Need to Know

### 1. **Enums** (Important!)
An enum is a type that can be ONE of several possible values. Think of it like a box that can contain different things, but only ONE thing at a time.

```rust
pub enum Token {
    Let,           // This is a variant with no data
    Number(i64),   // This variant holds a number
    Ident(String), // This variant holds text
    EOF,
}
```

Usage:
```rust
let token = Token::Number(42);    // Create a Number variant
let token2 = Token::Ident("x".to_string());  // Create an Ident variant
```

**Pattern Matching** with `match`:
```rust
match token {
    Token::Number(n) => println!("Got number: {}", n),
    Token::Ident(s) => println!("Got identifier: {}", s),
    Token::Let => println!("Got keyword let"),
    _ => println!("Something else"),
}
```

### 2. **Structs** (Like Classes)
A struct is a container that holds multiple pieces of data together:

```rust
pub struct Parser {
    lexer: Lexer,      // Field 1: a Lexer
    current: Token,    // Field 2: a Token
}
```

### 3. **impl Blocks** (Methods)
`impl` blocks attach methods (functions) to structs:

```rust
impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        // Constructor method
        Self { lexer, current: Token::EOF }
    }
    
    pub fn parse(&mut self) -> Vec<Stmt> {
        // Instance method (can access self)
    }
}
```

- `pub fn new()` - Constructor, creates a new instance
- `&mut self` - Mutable reference to self (can modify the struct)
- `Vec<Stmt>` - Vector (dynamic array) of Statements

### 4. **References and Borrowing**
Rust is strict about who owns data. Use `&` to borrow (read-only) or `&mut` to mutably borrow (read-write):

```rust
fn foo(s: &String) {      // Borrow - can't modify
}

fn bar(s: &mut String) {   // Mutable borrow - can modify
}
```

### 5. **Box<T>** (Pointer to Heap Memory)
When you need recursive structures (like nested expressions), you use `Box` to put data on the heap:

```rust
pub enum Expr {
    Binary {
        left: Box<Expr>,   // Points to another expression
        op: BinOp,
        right: Box<Expr>,  // Points to another expression
    },
}
```

Without `Box`, Rust can't know the size at compile time!

### 6. **Result<T, E>** (Error Handling)
Returns either `Ok(value)` or `Err(error)`:

```rust
fn check() -> Result<(), Diagnostic> {
    if something_wrong {
        return Err(Diagnostic::new("Error", "Description"));
    }
    Ok(())
}

// Usage:
if let Err(diag) = typechecker.check(&program) {
    println!("Error: {}", diag.message);
}
```

### 7. **Option<T>** (Maybe a Value)
Returns either `Some(value)` or `None`:

```rust
pub fn get(&self, name: &str) -> Option<Value> {
    if variable_exists {
        Some(value)
    } else {
        None
    }
}

// Usage:
match env.get("x") {
    Some(value) => println!("Found: {}", value),
    None => println!("Not found"),
}
```

### 8. **Traits and Derive**
`#[derive(Debug)]` automatically implements debugging. `Clone` makes copies, `PartialEq` allows `==`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Token { ... }
```

---

## Project Structure

```
axiom/
├── Cargo.toml              # Package configuration
├── src/
│   ├── main.rs            # Entry point - orchestrates the pipeline
│   ├── lexer.rs           # Tokenization (text → tokens)
│   ├── parser.rs          # Parsing (tokens → AST)
│   ├── ast.rs             # Abstract Syntax Tree definitions
│   ├── interpreter.rs     # Execution (AST → results)
│   ├── typechecker.rs     # Type checking (AST → types)
│   ├── types.rs           # Type definitions
│   └── diagnostic.rs      # Error messages
└── target/                # Build output (auto-generated)
```

---

## The Compilation Pipeline

When you run `cargo run`, here's what happens:

```
INPUT CODE
    ↓
[LEXER] - Converts text into tokens
    ↓
Token Stream (Let, Ident("t"), Equal, Now, ...)
    ↓
[PARSER] - Converts tokens into AST (Abstract Syntax Tree)
    ↓
AST (Stmt::Let { name: "t", value: Expr::Now }, ...)
    ↓
[TYPE CHECKER] - Verifies types are correct
    ↓
Types (verified)
    ↓
[INTERPRETER] - Executes the AST
    ↓
Result (The answer)
```

Let's trace through an example: `let t = now`

---

## Detailed Component Breakdown

### 1. **main.rs** - The Orchestrator

This is the entry point. It:
1. Defines the input code
2. Calls the lexer
3. Calls the parser
4. Calls the type checker
5. Calls the interpreter
6. Prints the result

```rust
fn main() {
    let input = "let t = now\nt";
    
    // Step 1: Tokenize
    let lexer = Lexer::new(input);
    
    // Step 2: Parse
    let mut parser = Parser::new(lexer);
    let program = parser.parse();
    
    // Step 3: Type check
    let mut typechecker = TypeChecker::new();
    if let Err(diag) = typechecker.check(&program) {
        print_diagnostic(diag);
        return;
    }
    
    // Step 4: Interpret
    let mut interpreter = Interpreter::new();
    let result = interpreter.execute(&program);
    
    println!("Result: {:?}", result);
}
```

### 2. **lexer.rs** - Tokenization

**Purpose**: Convert raw text into tokens (meaningful units).

**Token Enum**: Represents all possible tokens:
```rust
pub enum Token {
    Let,                // keyword
    Ident(String),      // variable name
    Number(i64),        // number literal
    Plus, Minus, Star, Slash, Equal, // operators
    EOF,                // end of file marker
}
```

**Lexer Struct**: Tracks position in input:
```rust
pub struct Lexer {
    input: Vec<char>,   // Input as array of characters
    pos: usize,         // Current position (0, 1, 2, ...)
}
```

**Key Methods**:

- `pub fn new(input: &str)` - Create lexer from string
  - Converts string to `Vec<char>` for easier character access
  
- `fn current() -> Option<char>` - Get current character without consuming it
  - Returns `Option` because we might be at EOF
  
- `fn advance()` - Move to next character
  
- `fn skip_whitespace()` - Skip spaces, newlines, tabs
  - Repeatedly calls `advance()` while on whitespace
  
- `fn read_number() -> i64` - Read a complete number
  - Example: input "123", reads all digits, returns 123
  
- `fn read_ident() -> String` - Read a complete identifier
  - Example: input "now", reads all alphanumeric chars, returns "now"
  
- `pub fn next_token() -> Token` - Get the next token
  - Skips whitespace
  - Checks current character
  - Returns appropriate token

**Example Trace**:
```
Input: "let x = 5"
Position 0, char 'l' → read_ident() → "let" → Token::Let
Position 3, char ' ' → skip_whitespace()
Position 4, char 'x' → read_ident() → "x" → Token::Ident("x")
Position 5, char ' ' → skip_whitespace()
Position 6, char '=' → Token::Equal
Position 7, char ' ' → skip_whitespace()
Position 8, char '5' → read_number() → 5 → Token::Number(5)
Position 9, end → Token::EOF
```

---

### 3. **parser.rs** - Parsing

**Purpose**: Convert token stream into Abstract Syntax Tree (AST).

**Parser Struct**: Tracks current token:
```rust
pub struct Parser {
    lexer: Lexer,      // Lexer to get more tokens
    current: Token,    // The token we're currently looking at
}
```

**Recursive Descent Parsing**: Each method parses one level of the grammar.

**Grammar Priority** (called in order, determines precedence):
1. `parse_expr()` - Top level
   ↓
2. `parse_add()` - Addition/Subtraction (lower precedence)
   ↓
3. `parse_mul()` - Multiplication/Division (higher precedence)
   ↓
4. `parse_primary()` - Numbers, variables, primitives (highest precedence)

**Why this order?** In math, multiplication has higher precedence than addition.
`2 + 3 * 4` means `2 + (3 * 4)` = 14, not `(2 + 3) * 4` = 20.

**Key Methods**:

- `pub fn parse() -> Vec<Stmt>` - Parse entire program
  - Loop through all tokens, parse each statement
  - Return vector of statements
  
- `fn parse_stmt()` - Parse one statement
  - If it's `let`, call `parse_let()`
  - Otherwise, it's an expression
  
- `fn parse_let() -> Stmt` - Parse variable assignment
  - Expect: `let identifier = expression`
  - Returns: `Stmt::Let { name, value }`
  
- `fn parse_expr()` - Parse expression (just calls `parse_add`)
  
- `fn parse_add()` - Parse addition/subtraction
  ```rust
  let mut expr = self.parse_mul();  // Get left side
  loop {
      if current token is + or - {
          create Binary expression
          get right side with parse_mul()
      } else {
          break
      }
  }
  ```
  
- `fn parse_mul()` - Parse multiplication/division (same pattern as add)
  
- `fn parse_primary()` - Parse bottom level
  - Numbers: return `Expr::Number(n)`
  - Identifiers: check if it's "now", return `Expr::Now` or `Expr::Var(name)`

**Example Trace**:
```
Input tokens: Let, Ident("t"), Equal, Now, EOF

parse()
└─ parse_stmt()
   ├─ Sees Token::Let
   └─ parse_let()
      ├─ Advance, see Ident("t"), name = "t"
      ├─ Advance, see Equal
      ├─ Advance, see Now
      ├─ parse_expr()
      │  └─ parse_add()
      │     └─ parse_mul()
      │        └─ parse_primary()
      │           └─ Return Expr::Now
      └─ Return Stmt::Let { name: "t", value: Expr::Now }
```

---

### 4. **ast.rs** - Abstract Syntax Tree

**Purpose**: Define the structure of the program after parsing.

```rust
pub enum Expr {
    Number(i64),          // Literal: 5, 42, -1
    Var(String),          // Variable: x, y, my_var
    Now,                  // Special: current time
    Binary { ... },       // Operation: 2 + 3, x * y
}

pub enum BinOp {
    Add, Sub, Mul, Div    // +, -, *, /
}

pub enum Stmt {
    Let { name, value },  // let x = expr
    Expr(Expr),          // Just an expression
}
```

**Why separate enums?**
- `Expr` = values and computations
- `Stmt` = declarations and statements
- `BinOp` = operators

---

### 5. **interpreter.rs** - Execution

**Purpose**: Execute the AST and compute results.

**Value Enum**: What variables can hold:
```rust
pub enum Value {
    Int(i64),    // Integer value: 5, -10, 999
    Time(i64),   // Time value: result of "now"
}
```

**Env Struct**: Variable storage:
```rust
pub struct Env {
    vars: HashMap<String, Value>,  // Maps "x" → Int(5)
}
```

**Interpreter Struct**: The executor:
```rust
pub struct Interpreter {
    env: Env,       // Variable environment
    time: i64,      // Global time counter
}
```

**Key Methods**:

- `pub fn new() -> Self` - Create new interpreter
  - Initialize with empty environment and time = 0
  
- `fn eval_expr(&mut self, expr: &Expr) -> Value` - Evaluate an expression
  - `Number(n)` → `Value::Int(n)`
  - `Var(name)` → Look up name in env
  - `Now` → Return `Value::Time(time)`, then increment time
  - `Binary { left, op, right }` → Evaluate both sides, apply operation
  
  Example: `Binary { 2 + 3 }`
  ```rust
  let l = eval_expr(2);       // Value::Int(2)
  let r = eval_expr(3);       // Value::Int(3)
  match (l, r, op) {
      (Int(2), Int(3), Add) => Value::Int(5)
  }
  ```
  
- `pub fn execute(&mut self, stmts: &[Stmt]) -> Option<Value>` - Run program
  - For each statement:
    - If it's `Let`, evaluate value and store in env
    - If it's `Expr`, evaluate and keep as last result
  - Return the last expression result

**Example Trace**:
```
Program: [Let { "t", Now }, Expr(Var("t"))]

execute()
├─ Stmt::Let { "t", Now }
│  ├─ eval_expr(Now)
│  │  ├─ Current time = 0
│  │  ├─ Return Value::Time(0)
│  │  └─ Increment time to 1
│  └─ Store in env: "t" → Time(0)
│
└─ Stmt::Expr(Var("t"))
   ├─ eval_expr(Var("t"))
   │  └─ Look up "t" in env → Some(Time(0))
   │     Return Time(0)
   └─ last = Some(Time(0))

Result: Some(Time(0))
```

---

### 6. **typechecker.rs** - Type Verification

**Purpose**: Verify that operations are valid before execution.

Example issues it catches:
- Can't do arithmetic on Time values
- Using undefined variables (might be included later)

```rust
pub struct TypeChecker {
    // Tracks known variables and their types
}

impl TypeChecker {
    pub fn check(&mut self, stmts: &[Stmt]) -> Result<(), Diagnostic> {
        // Verify all statements are type-safe
    }
}
```

---

### 7. **types.rs** - Type Definitions

```rust
pub enum Type {
    Int,    // Integer type
    Time,   // Time type
}
```

---

### 8. **diagnostic.rs** - Error Reporting

```rust
pub struct Diagnostic {
    pub title: String,      // "Type Error"
    pub message: String,    // "Cannot add Int and Time"
    pub help: Option<String>, // "Use separate variables"
}
```

---

## How to Run & Develop

### Running the Program
```bash
cd e:\programs\axiom
cargo run
```

The output shows: `Result: Some(Time(0))`

### Making Changes

**Want to add a new operator?**

1. **Add to Token enum** (lexer.rs)
   ```rust
   pub enum Token {
       // ...
       Percent,  // % for modulo
   }
   ```

2. **Update next_token()** (lexer.rs)
   ```rust
   Some('%') => { self.advance(); Token::Percent }
   ```

3. **Add to BinOp enum** (ast.rs)
   ```rust
   pub enum BinOp {
       // ...
       Mod,
   }
   ```

4. **Update parse_mul()** (parser.rs) - if same precedence as * and /
   ```rust
   Token::Percent => {
       self.advance();
       expr = Expr::Binary {
           left: Box::new(expr),
           op: BinOp::Mod,
           right: Box::new(self.parse_primary()),
       };
   }
   ```

5. **Update eval_expr()** (interpreter.rs)
   ```rust
   (Value::Int(a), Value::Int(b), BinOp::Mod) => Value::Int(a % b),
   ```

**Testing**: Just modify the input in main.rs and run `cargo run` again!

### Debugging

- **Print AST**: Add `println!("AST: {:#?}", program);` in main.rs
- **Print tokens**: Modify lexer to print each token
- **Step through**: Use `println!()` in interpreter functions

---

## Key Concepts Summary

| Concept | Purpose | Example |
|---------|---------|---------|
| **Enum** | Type with multiple variants | `Token::Let` or `Token::Number(5)` |
| **Struct** | Container for related data | `struct Parser { lexer, current }` |
| **impl** | Attach methods to structs | `impl Parser { fn new() {} }` |
| **match** | Pattern matching on enums | `match token { Token::Let => ... }` |
| **Box<T>** | Heap allocation for recursion | `left: Box<Expr>` |
| **Vec<T>** | Dynamic array | `Vec::new()`, `vec.push(item)` |
| **HashMap** | Key-value storage | `vars.get("x")`, `vars.insert("x", value)` |
| **Option<T>** | Maybe a value | `Some(5)` or `None` |
| **Result<T,E>** | Success or error | `Ok(result)` or `Err(error)` |

---

## Next Steps

1. Try adding new features (new operators, more keywords)
2. Add error messages for undefined variables
3. Add support for function definitions
4. Implement more complex types
5. Build a REPL (Read-Eval-Print Loop) for interactive use

Happy learning!
