
use fhe_stackmachine::fvm::{VM, Value, OpCode};

fn main() {
    let mut vm = VM::new();
    vm.push(Value::Int16(30000));
    vm.push(Value::Int8(100));
    vm.execute(vec![OpCode::Add]);
    // Expecting i128 because the operands were promoted
    assert_eq!(vm.pop(), Value::Int128(30100));
}
