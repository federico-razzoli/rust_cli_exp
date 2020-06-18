# Rust CLI Expriments (cli_exp)

This project exists to experiment with creating a set of CLI programs in Rust,
and possibly find out a set of good/bad practices.

This file documents what a maintiner should know about the project.

## Compiling

Note that we have multiple binaries, so we use Cargo slightly differently than usual.

```
# we can check the whole project, not a single bin:
cargo check
# we run a single bin:
cargo run --bin <bin-name>
# we can compile all bins or one of them:
cargo build
cargo build --bin mapper_day
```

bin-name is the binary name as it is written in Cargo.toml, not a filename. Example:

```
cargo run --bin mapper_day
```

## Credits

cli_exp is maintained by Federico Razzoli, see Cargo.toml.

However, it was originary forked from rust_multibin, by @timglabisch. He wrote the boilerplate
to compile multiple binaries from a single project.
