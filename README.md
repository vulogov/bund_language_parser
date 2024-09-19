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

# Bund language syntax

The BUND programming language follows a concatenative structure, wherein a program comprises a sequence of terms. Some of these terms denote data stored within a stack, while others reference functions that process the stack's data or issue commands to alter the state of the stack-based Virtual Machine.

```rust
2 2 + .
```

This program serves as a simple demonstration of some important BUND concepts. It begins by pushing two numbers to the current stack. Notably, the BUND virtual machine supports multiple stacks. Next, a function called "+" is invoked, which takes two elements from the current stack, performs arithmetic operations and then pushes the result back onto the stack. Following that, the function "." is called, which retrieves the last element from the current stack and moves it to the workbench stack.

One of the most essential elements of a computer program is the comment. BUND uses "//" (double slash) at the beginning of a comment and "end of line" to signify the end of a comment.

## Numbers

BUND supports two types of numeric values: integers and floats. Standard regular or scientific format for floats are supported.

```rust
42     // This is an example of integer, internally represented by i64
42.0   // This is an example of float, internally represented by f64
```

## String

String in bund is a sequence of ASCII or Unicode characters enclosed in "\"" symbols.

```rust
"This is an example of string"
```

You can also define a string that doesn't contain spaces by prepending it with ":"

```rust
:THISISALSOTHESTRING
```

Another form of string is literal, that is sequence of ANY characters enclosed in "\'"

```rust
'I am a literal'
```

## Function calls

References to functions that have to be immediately executed defined by the following rule ```( LETTER | SYMBOL | "." | "," | "=" | ">" | "<" | "-" | "+" | "^" | "?" | "!" | "/" | "*")+```

```rust
"Hello world!" println // This is call for execution of function "println"
```

## Function pointer

Function pointer refers the function without immediate execution.

```rust
"Hello world!" `println // Now, println is a pointer o the function and it will not be executed until direct intervention
```

## Commands

Here is the list of the commands that changing state of VM

| Command name | Description |
|---|---|
| : | Enable auto-add feature of the VM |
| ; | Disable auto-add feature of the VM |

## Lambda

Anything that is included between "{" and "}" is defined as the body of lambda function

```rust
{ 1 2 42.0 :Hello `println ! }
```

## List

Anything that is included between "[" and "]" is defined as the content of the list value

```rust
[ 1 2 42.0 :Hello `println ! ]
```

## Reference to named stack.

If you prepend any Unicode string that doesn't contains spaces with symbol "@", Bund parser will interpret this as a reference to named stack.

```rust
@Stackname
```

## Execution in separate context

Anything enclosed between "(" and ")" will be stored in separate anonymous stack. Stack will be deleted upon receiving ")" command.

```rust
( 42 . )  // I am placing 42 in a separate context, but then move it to a Workbench stack
```
