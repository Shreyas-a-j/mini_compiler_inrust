# Mini Compiler in Rust

A small compiler built from scratch in **Rust** to understand how compilers work internally.

## Compiler Pipeline

**Source File → Lexer → Tokens → Parser → AST → Semantic Analysis → IR → Code Generation**

## Current Progress

- [x] Read source code from files
- [x] Token definitions
- [x] Lexer
- [x] Keywords and identifiers
- [x] Integer literals
- [x] Operators
- [x] Strings
- [x] Comments
- [x] Lexer error handling
- [x] Line and column tracking
- [x] Parser foundation
- [x] Basic expression parsing
- [x] AST foundation
- [ ] Binary expressions
- [ ] Operator precedence
- [ ] Statements
- [ ] Name resolution
- [ ] Type checking
- [ ] Intermediate representation
- [ ] Code generation

## Goal

Build a working compiler from scratch while developing a deep understanding of:

- Lexing
- Parsing
- AST design
- Semantic analysis
- Type systems
- Intermediate representations
- Code generation
- Compiler architecture

The project is focused on **learning how each stage of a compiler works and how the stages connect together**.

## Status

**Currently working on:** Parser and AST