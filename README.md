# VYRON

**VYRON** is an experimental systems programming language focused on explicit effects, deterministic behavior, and compiler-enforced boundaries.

The project is being developed from first principles. It is not an AI framework: the goal is a small, inspectable language implementation with its own lexer, parser, semantic checks, compiler, and runtime.

## Current milestone — 0.1.0

This first milestone establishes the language front-end:
- Rust project and command-line entry point
- Lexer for identifiers, numeric and string literals, comments, and core punctuation
- Parser for modules, effect declarations, and function declarations
- Effect checker that rejects undeclared effects
- Unit tests for valid syntax and malformed input
- Minimal .vyr example

The current parser intentionally supports a narrow grammar. The primitive type-checking API currently validates literal return values for i64, bool, string, and unit, including i64 overflow. It is the first isolated type-system stage; parser-integrated expression typing, function parameters, bytecode generation, and a runtime remain future milestones.

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

## Roadmap

1. Front-end hardening: source spans, diagnostics, parser recovery, grammar tests.
2. Type system: expand primitive typing into parsed return expressions, function signatures, and expression typing.
3. Effect system: effect propagation, call-graph validation, capability boundaries.
4. Intermediate representation: typed IR and deterministic lowering.
5. Compiler: bytecode generation and validation.
6. Runtime: portable VM, memory model, and controlled I/O.
7. Developer experience: formatter, language server, package manager, and documentation.

## Design principles

- Explicit effects instead of hidden ambient capabilities
- Deterministic compilation and reproducible tests
- Small compiler stages with clear contracts
- No performance or safety guarantees without benchmarks and evidence

## License

MIT
