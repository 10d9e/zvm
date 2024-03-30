use fvm::vm::{OpCode, Value, VM};

fn main() {
    let mut vm = VM::new();

    let program = vec![
        OpCode::Push(Value::Int32(5)),  // Push 5 onto the stack
        OpCode::Push(Value::Int32(10)), // Push 10 onto the stack
        OpCode::Add,                    // Add the top two values on the stack
    ];

    vm.execute(&program);

    if let Some(result) = vm.stack.last() {
        println!("Result of addition: {:?}", result);
    } else {
        println!("Error: Stack is empty.");
    }
}
