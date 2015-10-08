js.rs — Parser
==============

This is the parser for js.rs, a sever-side interpreter for JavaScript being developed in Rust (WIP). It uses `lalrpop` as a parser generator.

Currently, only basic arithmetic expressions (addition, multiplication, and subtraction, and parenthetical grouping) for ints and floats are supported.

AST
---

The type definitions for the AST can be found in `src/ast.rs`. Since arithmetic instructions are the only things implemented right now, the only typs are `Exp` and `BinOp`.

Parsing
-------

Currently the only parsing function generated is `parse_Exp`, which takes a `&str` and returns an `Exp` (wrapped in a `Result`).
