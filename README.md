# Mini Compiler

A small compiler project written in **Rust**, built from scratch to understand how a compiler works internally.

The project is being developed incrementally, starting with a **lexer**, followed by a **parser** and **AST**.

---

## Project Goal

The goal of this project is to build a small compiler front-end in Rust:

```text
Source Code
     ↓
   Lexer
     ↓
   Tokens
     ↓
   Parser
     ↓
    AST
```

Eventually, the project will grow into a complete compiler pipeline.

For now, the focus is on understanding and implementing the lexer from first principles.

---

## Current Progress

### Day 1 — Lexer Foundations

The first stage of the project is the lexer.

The lexer takes source code such as:

```text
let x = 42;
```

and converts it into a stream of tokens:

```text
Let
Identifier("x")
Equal
Integer(42)
Semicolon
EOF
```

---

## Current Token Types

The lexer currently understands the following tokens:

### Keywords

```text
let
if
else
return
```

### Identifiers

Examples:

```text
x
name
counter
my_variable
hello123
```

Identifiers currently follow these rules:

```text
First character:
    ASCII letter or _

Remaining characters:
    ASCII letter, digit, or _
```

Examples:

```text
hello
hello123
_my_variable
counter_1
```

---

### Integer Literals

The lexer supports decimal integer literals:

```text
0
10
42
12345
```

These are converted into:

```rust
Token::Integer(i64)
```

---

### Operators

Currently implemented:

```text
=
+
```

---

### Delimiters

Currently implemented:

```text
;
```

---

### Special Token

The lexer also produces:

```text
EOF
```

to indicate that the input has been completely consumed.

---

## Current Lexer Architecture

The lexer maintains two pieces of state:

```rust
struct Lexer {
    input: Vec<char>,
    position: usize,
}
```

`input` contains the source code converted into characters.

`position` tracks the current location in the source.

Conceptually:

```text
let x = 42;
^
```

The `position` moves through the source as tokens are consumed.

---

## Token Representation

The current token enum is:

```rust
#[derive(Debug, PartialEq)]
enum Token {
    Let,
    If,
    Else,
    Return,

    Identifier(String),
    Integer(i64),

    Equal,
    Plus,
    Semicolon,

    EOF,
}
```

Tokens such as identifiers and integers carry additional data.

For example:

```rust
Token::Identifier("counter".to_string())
```

and:

```rust
Token::Integer(42)
```

---

## Lexer Functions

The lexer currently contains the following important functions.

### `new()`

Creates a lexer from source code:

```rust
fn new(input: &str) -> Self
```

Example:

```rust
let mut lexer = Lexer::new("let x = 42;");
```

---

### `next_token()`

The main lexer function:

```rust
fn next_token(&mut self) -> Token
```

Its job is to:

1. Skip irrelevant whitespace.
2. Check for EOF.
3. Inspect the current character.
4. Determine what type of token begins there.
5. Consume the characters belonging to that token.
6. Return the token.

---

### `skip_whitespace()`

Whitespace is ignored by the lexer.

For example:

```text
let    x     =     42;
```

and:

```text
let x = 42;
```

should eventually produce the same token stream.

---

### `read_identifier()`

Consumes an entire identifier:

```text
counter123
```

rather than only the first character.

The general process is:

```text
start
  ↓
consume identifier characters
  ↓
stop at first non-identifier character
  ↓
create String
```

---

### `lookup_keyword()`

After an identifier-like sequence is read, the lexer checks whether it is a language keyword.

For example:

```text
let
```

becomes:

```rust
Token::Let
```

while:

```text
letter
```

becomes:

```rust
Token::Identifier("letter".to_string())
```

This is important because keywords must be recognized as complete lexemes.

---

### `read_number()`

Consumes consecutive decimal digits:

```text
12345
```

and converts them into:

```rust
Token::Integer(12345)
```

---

## Example

Given:

```text
let my_variable = 42;
```

the lexer should produce:

```text
Let
Identifier("my_variable")
Equal
Integer(42)
Semicolon
EOF
```

---

## Current Limitations

The lexer is still intentionally incomplete.

It does **not** currently support:

```text
==
!=
<
>
<=
>=
```

These will be implemented using **lookahead**.

It also does not yet have proper lexer error handling.

For example:

```text
let x = @;
```

currently does not produce a proper lexer error.

That will be fixed later.

The lexer also currently supports only integer literals, not:

```text
3.14
0xff
1_000
```

Those features may be considered later depending on the language design.

---

## Architecture Being Developed

The project will eventually be structured approximately like this:

```text
mini_compiler/
│
├── Cargo.toml
│
├── src/
│   ├── main.rs
│   ├── lexer.rs
│   ├── token.rs
│   ├── parser.rs
│   ├── ast.rs
│   └── error.rs
│
└── tests/
    ├── lexer_tests.rs
    └── parser_tests.rs
```

At the beginning, everything is intentionally kept small so that the compiler concepts can be understood before splitting the project into modules.

---

## Compiler Roadmap

### Phase 1 — Lexer

```text
Source Code
     ↓
Characters
     ↓
Lexer
     ↓
Tokens
```

Current focus.

---

### Phase 2 — Parser

```text
Tokens
   ↓
Parser
   ↓
AST
```

The parser will understand the grammatical structure of the language.

Example:

```text
10 + 20 * 3
```

should be parsed according to operator precedence.

---

### Phase 3 — AST

The parser will construct an Abstract Syntax Tree.

Example:

```text
10 + 20 * 3
```

will eventually become something conceptually like:

```text
      +
     / \
   10   *
       / \
     20   3
```

---

### Phase 4 — Compiler Front-End

The complete front-end will become:

```text
Source Code
     ↓
   Lexer
     ↓
   Tokens
     ↓
   Parser
     ↓
    AST
```

---

## Development Philosophy

This project is being built incrementally.

For each feature:

```text
Understand
    ↓
Design
    ↓
Implement
    ↓
Run
    ↓
Test
    ↓
Break intentionally
    ↓
Fix
    ↓
Refactor
```

The goal is not simply to produce a working compiler.

The goal is to understand **why the compiler works** and how each component interacts with the next.

---

## Current Milestone

### Completed

```text
✅ Rust project created
✅ Token enum created
✅ Lexer struct created
✅ Lexer constructor
✅ Character-by-character scanning
✅ Whitespace skipping
✅ Identifier recognition
✅ Keyword recognition
✅ Integer recognition
✅ `=` token
✅ `+` token
✅ `;` token
✅ EOF handling
```

### Next

```text
→ Lookahead
→ == 
→ !=
→ <
→ >
→ <=
→ >=
→ Better lexer errors
→ Lexer tests
→ Refactoring into modules
```

---

## Long-Term Goal

By the end of the project, the compiler should be able to take a small program such as:

```text
let x = 10;

if x > 5 {
    return x + 20;
}
```

and transform it through:

```text
SOURCE
  │
  ▼
LEXER
  │
  ▼
TOKENS
  │
  ▼
PARSER
  │
  ▼
AST
```

with each stage implemented and understood in Rust from scratch.
