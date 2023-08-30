# dike/vm

The virtual machine that executes dike vm instructions

## Components

**Value**

`value` is an enum that represents a single item on the stack. There are two different classes of values:

_Imitable_ values are never stored on the heap, but only copied around on the stack. That isn't to say that they are never placed on the
heap, they might be if they are behind a variable reference. _Inimitable_ values are always stored on the heap, and never on the stack. They
can only be stored on the stack by way of the imitable value, `Ref`, which stores an imitable pointer to the heap location where the value
is stored.

**Rust Interop**

To keep things simple (and efficient), all but a few operations are done in Rust. Rust interop is more deeply covered in the `interop`
module, but the basics are here.

- `interop_gen.rs` is a generated file. Currently, it is written to handle some basic operations and rust values for testing, but it is
  meant to be overwritten.
- Most operations and values correspond to rust operations and values. The only exceptions are the operations that are built into the
  language (if, give, etc.).

**Operations**

No matter how many times a value is referenced, either by variables, or due to it's inimitable nature, it is tracked down and "borrowed"
before an instruction is executed on it. "borrowing" in this sense does not necessarily mean "borrowed" the rust way, as `Memory` isn't
really "rusty" (`RefCell` is used). Instead, the value is swapped in the heap with `undefined` and then swapped back once the operation is
over. This allows the interop code to give the rust operator functions the exact flavor of the value that they ask for, be it `&Value` or
`&mut Value`. It is illegal to pass an owned value into a rust function because it might not return the ownership back out, which could make
it so that the `undefined` can not be swapped back on the heap after the operation is complete.
