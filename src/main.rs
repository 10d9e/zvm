use fvm::vm::{OpCode, Value, VM};

fn main() {
    // Example usage of the VM
    let mut vm = VM::new();
    vm.push(Value::Int8(30));
    vm.push(Value::Int16(40));
    vm.execute(&[OpCode::Mul]);
    println!("{:?}", vm.stack); // Expected output: [Value::Int16(1200)]
}
