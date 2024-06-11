# ZVM
A virtual machine for building and executing Fully Homomorphic programs.

# Stack Machine Operations

This document describes the stack machine's operations, defined by the `OpCode` enum. The stack machine supports various arithmetic, bitwise, comparison, and stack manipulation operations. Additionally, it includes load and store operations for addressable memory space.

## Table of Contents
- [Arithmetic Operations](#arithmetic-operations)
- [Bitwise Operations](#bitwise-operations)
- [Comparison Operations](#comparison-operations)
- [Stack Manipulation Operations](#stack-manipulation-operations)
- [Memory Operations](#memory-operations)
- [Miscellaneous Operations](#miscellaneous-operations)
- [Serialization and Deserialization](#serialization-and-deserialization)
- [VM Structure](#vm-structure)
- [VM Methods](#vm-methods)

## Arithmetic Operations
These operations perform basic arithmetic on the top elements of the stack.

- `Add`: Pop the top two elements, add them, and push the result.
- `Sub`: Pop the top two elements, subtract the second from the first, and push the result.
- `Mul`: Pop the top two elements, multiply them, and push the result.
- `Div`: Pop the top two elements, divide the first by the second, and push the result.
- `Neg`: Pop the top element, negate it, and push the result.
- `Rem`: Pop the top two elements, compute the remainder of their division, and push the result.

## Bitwise Operations
These operations perform bitwise manipulation on the top elements of the stack.

- `And`: Pop the top two elements, perform bitwise AND, and push the result.
- `Or`: Pop the top two elements, perform bitwise OR, and push the result.
- `Xor`: Pop the top two elements, perform bitwise XOR, and push the result.
- `ShiftRight`: Pop the top two elements, shift the first right by the second, and push the result.
- `ShiftLeft`: Pop the top two elements, shift the first left by the second, and push the result.

## Comparison Operations
These operations compare the top elements of the stack.

- `Eq`: Pop the top two elements, compare them for equality, and push the result.
- `Neq`: Pop the top two elements, compare them for inequality, and push the result.
- `Lt`: Pop the top two elements, check if the first is less than the second, and push the result.
- `Lte`: Pop the top two elements, check if the first is less than or equal to the second, and push the result.
- `Gt`: Pop the top two elements, check if the first is greater than the second, and push the result.
- `Gte`: Pop the top two elements, check if the first is greater than or equal to the second, and push the result.
- `Min`: Pop the top two elements, push the minimum of them.
- `Max`: Pop the top two elements, push the maximum of them.

## Stack Manipulation Operations
These operations directly manipulate the stack.

- `Push(Value)`: Push a value onto the stack.
- `Dup`: Duplicate the top item on the stack.
- `Swap`: Swap the top two elements of the stack.
- `Inc`: Increment the top element by one.
- `Dec`: Decrement the top element by one.
- `NoOp`: No operation, does nothing.

## Memory Operations
These operations interact with the memory space of the VM.

- `Load(i32)`: Load a value from memory at the given address and push it onto the stack.
- `Store(i32)`: Pop the top value from the stack and store it in memory at the given address.

## Mux Operation
- `Mux`: Pop three elements and perform a multiplexer operation using the first element as the condition and the next two as the possible values to select from.

## Example Usage

```rust
use zvm::value::Value;
use zvm::VM;
use zvm::OpCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    let code = "
        let a = 10;
        let b = 20;
        let a = a + b;
        if (a > 25) {
            let a = a - 5;
        }
    ";
    // parse the abstract syntax tree
    let ast = parse(code);
    // compile into homomorphic bytecode
    let compiled_opcodes = compile(&ast);

    // -- server -- 
    // On the server side set the server key and send over the compiled bytecode
    set_server_key(server_keys);

    // execute the bytecode
    let mut vm = VM::new();    
    vm.execute(&compiled_opcodes);

    if let Some(result) = vm.stack.last() {
        println!("Result of operation: {:?}", result);
    } else {
        println!("Error: Stack is empty.");
    }
    let encrypted_res = vm.stack.last();
    let clear_res: u8 = encrypted_res.as_eint8().decrypt(&client_key);
    println!("results: {}", clear_res);
    Ok(())
}
```

This example creates a VM, defines a program to add two numbers, executes the program, and prints the result.