# Dike

An attempt at building a programming language

## Goals

- A moderately fast scripting language
- Garbage-collected
- Strict type system
- Ability to run JIT or compiled
- Module-specific syscall permission system, allowing virtualization
- First-class support for Rust structs and functions, through which all IO is done
- Strict "mutation" rules, which enable safe concurrency on more than one OS thread
- Typesafe lazy loading of modules

## Architecture

The `parser` directory builds an abstract syntax tree (AST) from the source code.

The `interpreter` turns the AST into an _instruction graph_, which is, in turn, optimized by the `optimizer`.

The nitty-gritty stuff is in the `vm` directory. There, a rust process executes the intstructions generated by the intrepreter.

Ideally, more modules are to follow. But these will come after the core modules `parser`, `interpreter`, `optimizer`, and `vm` are at an MVP level.

## Development

If you are running the VM, you'll need to install Rust. If not, Deno and Dirt should be enough.

```shell
dirt --test # run all the tests

# to run just the tests for a certain module
dirt --test parser # or vm, etc.
```

### Testing the Parser

The parser tests a source file against several json files representing the expected AST, tokens, and diagnostics. Because of this beefy system a few helper utilities have been created, listed below:

```shell
# Overwrite the expected ast, tokens, and diagnostics with whatever the parser currently produces from the source
dirt --fill-parser-test additive_expression # or number_literal, comparison_expression, etc.
dirt --fill-parser-tests # fill all the parser tests

# Creates a new parser test with the given name and source
dirt --add-parser-test additive_expression "2 + 4"
```

