# Introduction

This Rust crate presents a parser for the Bund programming language, a concatenative language. The parser accepts a sequence of Bund language terms as input and generates a Vector of corresponding rust_dynamic::Value structures while also handling error conditions.

## Show me the code!

First, lest's take a look at classic HelloWorld program written in BUND.

```rust
//
// This is BUND Hello World program
//
"Hello world!" println
```
First, we can see that comments are denoted by "//" Next, we place the string "Hello world!" on the stack and then call the function "println" to take the string from the stack and print it. In the next step, we parse the source code.

```rust
let res = bund_parse(source).expect("Fail to parse BUND");
```
Here is the result: a list of rust-dynamic::Value elements. The first element in the list is a string (dt = 4). The next element is a CALL (dt = 6); the final element is an Exit element that shall stop the Virtual Machine.

```
[
  Value { id: "EmOYEaCNb_dneaUouRhPJ", stamp: 1726513449003.0, dt: 4, q: 100.0, data: String("Hello world!"), attr: [], curr: -1, tags: {} },
  Value { id: "mVYs--tyRe0efdpimoXA3", stamp: 1726513449003.0, dt: 6, q: 100.0, data: String("println"), attr: [], curr: -1, tags: {} },
  Value { id: "iPuCiciNSd__p1jKMObZf", stamp: 1726513449003.0, dt: 93, q: 100.0, data: Exit, attr: [], curr: -1, tags: {} }
]
```
