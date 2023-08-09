# Custom Interpreter 

This is currently a simple interpreter that can interpret arthimetic with white spaces and normal order of operations.  The easiest way to see what can currently be interpreted by this interpreter correctly is to look in `main.rs` and see tests there.  I will never merge any tests into the main branch unless they pass and the feature is complete, and I will likely not always update this README.


## Dependencies

There are no dependencies except those required to run Rust and its manager Cargo.  I don't use any external dependencies, just the Rust standard library.


## Feature List and Wish List

The goal is to expand this interpreter and improve the design so that it eventually runs a custom language or a mainstream language like Python, Javascript, or a joke language like DreamBird.


Two notes on the features list:
1. As mentioned above, the best way to see what it implemented is to check the tests in `main.rs`.  I will never merge the tests into main unless they pass and I will alwasy write the tests before starting the feature, so that will always show the most recent additions.  I will not always update the README.
2. The ultimate goal is to create a custom language or interpet a mainstream one, therefore this section will change heavily once the basic interpreter is complete and I decide the path to go down.

| Feature      | Status |
| ----------- | ----------- |
| Basic Arithmetic  | ✅ |
| Order of Ops      | ✅ |
| Loose Lexer and Interpeter Architecture | ✅ |
| Abstarct Syntax Tree | ⌛️ |
| Unary Operators and Support for Parathesis | ❌ |
| Opening and handling file | ❌ |


## Running Program

For the test suite, it is the usual cargo test command:
```
cargo test
```

Reading from a file is low on my priority list, so you will need to check if I have implemented it, but the command is also the usual cargo run:
```
cargo run -- <file name>
```