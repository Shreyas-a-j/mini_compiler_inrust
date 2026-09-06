# Mini Compiler

A small compiler project written from scratch in **Rust**.

The purpose of this project is to understand compiler construction by implementing the major stages ourselves rather than relying on parser/lexer generator libraries.

The project currently focuses on the **lexical analysis (lexer)** stage.

---

# Project Goal

The long-term goal is to build a small compiler front-end:

```text
Source Code
     │
     ▼
   Lexer
     │
     ▼
  Tokens
     │
     ▼
  Parser
     │
     ▼
   AST
```

Eventually, the project can be extended toward semantic analysis, intermediate representations, code generation, and other compiler stages.

For this month, the primary goal is to complete a reliable **lexer + parser + AST pipeline**.

---

# Current Status

## Day 1 — Lexer Foundations ✅

The first major component of the compiler is now working: the lexer.

For example:

```text
let x = 10 + 20;
```

is transformed into:

```text
Let
Identifier("x")
Equal
Integer(10)
Plus
Integer(20)
Semicolon
EOF
```

The lexer now handles:

* Keywords
* Identifiers
* Integer literals
* Arithmetic operators
* Assignment operators
* Comparison operators
* Braces and delimiters
* Whitespace
* End of file
* Basic lexical errors
* Lookahead for multi-character operators

---

# What Is a Lexer?

A lexer converts raw source code into a sequence of meaningful **tokens**.

For example:

```text
let counter = 42;
```

starts as characters:

```text
l e t   c o u n t e r   =   4 2 ;
```

The lexer groups those characters into lexemes:

```text
let
counter
=
42
;
```

and classifies them:

```text
LET
IDENTIFIER("counter")
EQUAL
INTEGER(42)
SEMICOLON
```

A useful distinction is:

```text
Lexeme = the actual text in the source

Token = the structured meaning assigned to that text
```

---

# Current Token Types

The current language supports the following token types.

## Keywords

```text
let
if
else
return
```

Examples:

```text
let
if
else
return
```

These are converted to dedicated token variants instead of identifiers.

---

## Identifiers

Examples:

```text
x
name
counter
hello123
my_variable
```

Identifier rules currently are:

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

## Integer Literals

The lexer currently supports decimal integers:

```text
0
10
42
12345
```

These become:

```rust
Token::Integer(42)
```

The numeric value is stored as an `i64`.

---

# Operators

## Arithmetic operators

```text
+
-
*
/
```

## Assignment

```text
=
```

## Equality and comparison

```text
==
!=
<
>
<=
>=
```

The distinction between:

```text
=
```

and:

```text
==
```

is handled using **lookahead**.

---

# Delimiters

Currently implemented:

```text
;
{
}
```

These will become more important once the parser starts handling statements and blocks.

---

# Whitespace

Whitespace is skipped by the lexer.

For example, these:

```text
let x = 42;
```

and:

```text
let     x    =    42;
```

produce the same tokens.

The lexer handles whitespace before attempting to identify the next token.

---

# Lexer Architecture

The lexer maintains its own scanning state:

```rust
pub struct Lexer {
    input: Vec<char>,
    position: usize,
}
```

Conceptually:

```text
let x = 42;
^
```

The `position` moves through the source as tokens are consumed.

---

# Main Lexer Operations

## `new()`

Creates a lexer from source code:

```rust
let mut lexer = Lexer::new("let x = 42;");
```

---

## `next_token()`

This is the main entry point for lexical analysis.

Conceptually:

```text
next_token()
     │
     ▼
skip whitespace
     │
     ▼
check EOF
     │
     ▼
inspect current character
     │
     ├── operator
     ├── identifier / keyword
     ├── number
     ├── delimiter
     └── invalid character
```

It returns either:

```rust
Ok(Token)
```

or:

```rust
Err(LexerError)
```

---

## `read_identifier()`

Consumes all characters belonging to an identifier.

For example:

```text
counter123
```

is consumed as one unit rather than:

```text
c
o
u
n
...
```

After reading the identifier, the lexer checks whether it is a keyword.

---

## `lookup_keyword()`

Determines whether an identifier-like sequence is actually a reserved keyword.

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
Token::Identifier("letter")
```

This means the lexer recognizes the complete lexeme before determining its token type.

---

## `read_number()`

Consumes consecutive digits:

```text
12345
```

and converts them into:

```rust
Token::Integer(12345)
```

---

## `peek()`

Used for one-character lookahead.

For example:

```text
==
```

When the lexer sees the first `=`, it checks the next character.

```text
current = '='
next    = '='
```

Therefore:

```rust
Token::EqualEqual
```

Similarly:

```text
=
```

produces:

```rust
Token::Equal
```

This same mechanism is used for:

```text
!=
<=
>=
```

---

# Error Handling

The lexer now distinguishes between:

```text
EOF
```

and:

```text
invalid character
```

For example:

```text
let x = @;
```

should not silently become `EOF` when `@` is encountered.

The current error representation is:

```rust
#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnexpectedCharacter(char),
}
```

The lexer therefore returns:

```rust
Err(LexerError::UnexpectedCharacter('@'))
```

for unsupported characters.

More detailed error information will be added later.

---

# Current Project Structure

The project is now being separated into modules:

```text
mini_compiler/
│
├── Cargo.toml
│
├── src/
│   ├── main.rs
│   ├── lexer.rs
│   └── token.rs
│
└── tests/
    └── lexer_tests.rs
```

The project will eventually grow into:

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

---

# Example

Given:

```text
let x = 10 + 20;

if x >= 50 {
    return x;
}
```

the lexer should recognize the tokens:

```text
Let
Identifier("x")
Equal
Integer(10)
Plus
Integer(20)
Semicolon

If
Identifier("x")
GreaterEqual
Integer(50)
LeftBrace

Return
Identifier("x")
Semicolon

RightBrace
EOF
```

The lexer does **not** determine what the program means.

It only identifies the pieces.

The parser will determine how those pieces are related.

---

# Important Compiler Concept

The lexer does not understand expressions.

For example:

```text
x + 10 * 3
```

The lexer produces:

```text
Identifier("x")
Plus
Integer(10)
Star
Integer(3)
```

It does not decide that multiplication has higher precedence than addition.

That is the parser's responsibility.

So the separation is:

```text
Lexer:
"What are these pieces?"

Parser:
"How are these pieces structured?"
```

---

# Development Workflow

Each compiler feature is being developed using this cycle:

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

The goal is not only to get a working compiler, but to understand why each component works.

---

# Roadmap

## Phase 1 — Lexer

```text
Source Code
     ↓
Characters
     ↓
Lexer
     ↓
Tokens
```

### Completed

```text
✅ Token representation
✅ Lexer state
✅ Character scanning
✅ Whitespace skipping
✅ Identifiers
✅ Keywords
✅ Integer literals
✅ Arithmetic operators
✅ Assignment
✅ Comparison operators
✅ Lookahead
✅ Braces
✅ EOF
✅ Basic lexer errors
✅ Initial module separation
```

### Remaining lexer work

```text
→ Strings
→ Comments
→ Line/column tracking
→ Better error messages
→ More lexer tests
→ Lexer cleanup/refactoring
```

---

# Phase 2 — Parser

After the lexer is stable:

```text
Tokens
   ↓
Parser
   ↓
AST
```

The parser will eventually understand:

```text
Variable declarations
Expressions
Binary operators
Operator precedence
Blocks
If/else
Return statements
Functions
```

---

# Phase 3 — AST

The parser will construct an Abstract Syntax Tree.

For:

```text
10 + 20 * 3
```

the AST should conceptually represent:

```text
      +
     / \
   10   *
       / \
     20   3
```

This structure captures the meaning of the expression.

---

# Phase 4 — Compiler Front-End

The target front-end pipeline is:

```text
Source Code
     │
     ▼
   Lexer
     │
     ▼
  Tokens
     │
     ▼
  Parser
     │
     ▼
    AST
```

---

# One-Month Goal

The month is structured approximately as:

```text
Week 1
Lexer fundamentals

Week 2
Parser fundamentals

Week 3
AST + language constructs

Week 4
Integration, errors, tests, and cleanup
```

The exact pace can change depending on how deeply we need to explore each concept.

---

# Current Milestone

```text
DAY 1 COMPLETE ✅
```

The first version of the lexer is working and the fundamental concepts behind lexical analysis have been implemented in Rust.

Next milestone:

```text
DAY 2
Strings
Comments
Source locations
Better errors
More thorough tests
```

---

# Long-Term Example

Eventually, the compiler should be able to accept a small program such as:

```text
let x = 10;

if x > 5 {
    return x + 20;
}
```

and process it through:

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

with every stage implemented and understood from scratch in Rust.
