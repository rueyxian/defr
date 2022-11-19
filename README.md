# defr

Golang `defer` statements but in Rust.

## Overview

A simple library that provides a convenient macro for wrapping expressions with the `drop` method.

## What is defer?

The meaning might vary but in the Golang context, `defer` is the [finalizer](https://en.wikipedia.org/wiki/Finalizer). Unlike C++ and Rust, Golang does not have destructor equivalents. [More on defer statements.](https://go.dev/ref/spec#Defer_statements)

## Difference

`defer` calls in Golang are pushed onto a stack, then run in a last-in-first-out manner.

```
func main() {
    defer run_4th()
    defer run_3rd()
    run_1st()
    run_2nd()
}
```

The expressions inside `defr!` block remain procedural. 

```
fn main() {
    defr! {
        run_3rd();
        run_4th();
    }
    run_1st();
    run_2nd();
}
```

## Notes

Definitely not a replacement for implementing the `Drop` trait. In fact, in most cases, the [RAII pattern](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization) is the way.