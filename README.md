js.rs — Parser
==============

This is the parser for [js.rs](https://github.com/rustoscript/js.rs), a sever-side interpreter for JavaScript currently being developed in Rust. It uses `lalrpop` as a parser generator.

Currently, it supports arithmetic expressions, variable declaration and assignment, and both pre- and post-incrementation/decrementation with `++` and `--`. The only values it currently can parse are numbers and `undefined`.

AST
---

The type definitions for the AST can be found in `src/ast.rs`. These types currently include expression (`Exp`), binary operators (`BinOp`), and statements (`Stmt`);

Parsing
-------

The two generated parser functions are `parse_Stmt` and `parse_Exp`, each of which both returns `Result` wrapping its respective AST value.
