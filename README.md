js.rs — Parser [![Build Status](https://travis-ci.org/rustoscript/js.rs-parser.svg)](https://travis-ci.org/rustoscript/js.rs-parser)
====================================================================================================================================

This is the parser for [js.rs](https://github.com/rustoscript/js.rs), a sever-side interpreter for JavaScript currently being developed in Rust. It makes use of [lalrpop](https://github.com/nikomatsakis/lalrpop), a parser generator.

Currently, it supports arithmetic expressions and boolean expressions, variable declaration and assignment, and `if` and `while` statements.

AST
---

The type definitions for the AST can be found in [js.rs-common](https://github.com/rustoscript/js.rs-common) in the `ast` module.

Parsing
-------

The two generated parser functions are `parse_Stmt` and `parse_Exp`, each of which a return a `Result` wrapping its respective AST value.
