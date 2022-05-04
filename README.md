## RUST

Statically-typed
Memory safe
Highly performant

###### https://www.rust-lang.org/tools/install

rustup - a tool for managing different versions of rust. (https://rust-lang.github.io/rustup/)
rustc - a tool for compipling Rust code.
cargo - a tool for managing Rust packages and projects

Commands

cargo run : build
cargo run -q : status are removed from the console

by default, varibles are immutable.
To be able to mutate varibales we must be explicitly declared with the MUT keyword

##### Primitive Data Types in Rust (https://www.codingame.com/playgrounds/365/getting-started-with-rust/primitive-data-types)

### Macros
Macro is a feature in rust for Metprogramming (it's when a program writes another program).
Support:
  Variadic arguments
  Pretty syntax
  Metaprogramming

Match expressions: comparison against a series of patterns

### Structures
Rust enforces a strict model at structures
We must explicitely define the types of our fields

### How does Rust make memory safety

A memory leak is when a program fails to keep track of its data.
Values assigned to a variable are owned by that variable.
So, when a variable is not needed it will be dropped.

By dfalut it can transfer ownership accross functions.
Or we can borrow the value using the & symbol

### Result type
Valid value or an error.
The purpose of the result type is to handle actions that can produce an error

### Binary Projects
Executable program. Cargo generates binary projects
$ cargo init

### Library Projects
Dependency for other programs.
$ cargo init --lib

### Bundling Code
It will need the entry point.
Output for the bundle.

### Why fileReader

Instead of transfering binary data we transfer a string since it is easier for transfer.
Also easier to transfer files between languages.

### Compile the Rust to web-assembly in order to use in the browser
@wasm-tool/wasm-pack-plugin