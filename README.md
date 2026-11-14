# VYRON

**VYRON** is an experimental systems programming language focused on explicit effects, deterministic behavior, and compiler-enforced boundaries.

The project is being developed from first principles. It is not an AI framework: the goal is a small, inspectable language implementation with its own lexer, parser, semantic checks, compiler, and runtime.

## Current milestone — 0.1.0 (early compiler prototype)

This first milestone establishes the language front-end:
- Rust project and command-line entry point
- Lexer for identifiers, numeric and string literals, comments, and core punctuation
- Parser for modules, effect declarations, zero-argument functions, and return expressions
- Integer expression parser with precedence, parentheses, and unary minus
- Effect checker that rejects undeclared effects
- Parser-integrated return type checking for i64, bool, string, and unit
- Constant-expression checks for i64 overflow and division by zero
- Typed intermediate representation (IR) with integer arithmetic instructions
- CLI commands for source checking and IR inspection
- Unit and integration tests for syntax, type errors, expression semantics, and IR lowering
- Minimal .vyr examples

The grammar is deliberately small: functions currently take no parameters and support one return expression. Local variables, control flow, function calls, bytecode generation, and a runtime remain future milestones.

## Syntax preview

    module hello;
    effect io;
    fn main() -> i64 !{io} {
        return 0;
    }

Effects are declared at module scope and listed explicitly on functions. The checker rejects a function that names an effect the module has not declared.

## Build

Requires the Rust stable toolchain.

    cargo build
    cargo test
    cargo run -- check examples/hello.vyr
    cargo run -- ir examples/hello.vyr
    cargo run -- check examples/arithmetic.vyr
    cargo run -- ir examples/arithmetic.vyr

## Roadmap

1. Front-end hardening: source spans, diagnostics, parser recovery, grammar tests.
2. Type system: parameters, variables, comparisons, logical operators, and full expression typing.
3. Effect system: effect propagation, call-graph validation, capability boundaries.
4. Intermediate representation: complete typed AST, control-flow blocks, and deterministic lowering.
5. Compiler: bytecode generation, verifier, and executable artifact format.
6. Runtime: portable VM, memory model, and controlled I/O.
7. Developer experience: formatter, language server, package manager, and documentation.

## Design principles

- Explicit effects instead of hidden ambient capabilities
- Deterministic compilation and reproducible tests
- Small compiler stages with clear contracts
- No performance or safety guarantees without benchmarks and evidence

## License

MIT
