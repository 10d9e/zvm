#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    // arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // bitwise operations
    And,
    Or,
    Xor,
    ShiftRight,
    ShiftLeft,

    // comparison
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    Min,
    Max,

    // multiplex
    Mux,

    // Jump
    Jmp,   // Jump to an instruction index unconditionally
    JmpIf, // Jump if the top of the stack is nonzero (true)

    Push(Value), // Push now carries a Value with it
    Dup, // Duplicate the top item on the stack
    NoOp, // No operation
}

impl OpCode {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            OpCode::Add => vec![0],
            OpCode::Sub => vec![1],
            OpCode::Mul => vec![2],
            OpCode::Div => vec![3],
            OpCode::And => vec![4],
            OpCode::Or => vec![5],
            OpCode::Xor => vec![6],
            OpCode::ShiftRight => vec![7],
            OpCode::ShiftLeft => vec![8],
            OpCode::Eq => vec![9],
            OpCode::Neq => vec![10],
            OpCode::Lt => vec![11],
            OpCode::Lte => vec![12],
            OpCode::Gt => vec![13],
            OpCode::Gte => vec![14],
            OpCode::Min => vec![15],
            OpCode::Max => vec![16],
            OpCode::Mux => vec![17],
            OpCode::Jmp => vec![18],
            OpCode::JmpIf => vec![19],
            OpCode::NoOp => vec![30],
            OpCode::Dup => vec![31],
            // Assign unique bytes to each OpCode...
            OpCode::Push(value) => {
                let mut bytes = vec![32]; // Example byte for Push
                bytes.extend(value.to_bytes());
                bytes
            },
        }
    }
}

impl OpCode {
    fn from_bytes(bytes: &[u8]) -> (Self, usize) { // Returns OpCode and bytes consumed
        match bytes[0] {
            0 => (OpCode::Add, 1),
            1 => (OpCode::Sub, 1),
            2 => (OpCode::Mul, 1),
            3 => (OpCode::Div, 1),
            4 => (OpCode::And, 1),
            5 => (OpCode::Or, 1),
            6 => (OpCode::Xor, 1),
            7 => (OpCode::ShiftRight, 1),
            8 => (OpCode::ShiftLeft, 1),
            9 => (OpCode::Eq, 1),
            10 => (OpCode::Neq, 1),
            11 => (OpCode::Lt, 1),
            12 => (OpCode::Lte, 1),
            13 => (OpCode::Gt, 1),
            14 => (OpCode::Gte, 1),
            15 => (OpCode::Min, 1),
            16 => (OpCode::Max, 1),
            17 => (OpCode::Mux, 1),
            18 => (OpCode::Jmp, 1),
            19 => (OpCode::JmpIf, 1),
            30 => (OpCode::NoOp, 1),
            31 => (OpCode::Dup, 1),
            // Interpret unique bytes back into OpCode...
            32 => {
                let (value, size) = Value::from_bytes(&bytes[1..]);
                (OpCode::Push(value), size + 1)
            },
            // Handle other opcodes...
            _ => unimplemented!(),
        }
    }
}


macro_rules! impl_ops {
    ($($op:ident, $op_method:ident, $op_token:tt);*) => {
        impl Value {
            $(
                fn $op(self, other: Self) -> Self {
                    match (self, other) {
                        (Value::Int8(a), Value::Int8(b)) => Value::Int8(a $op_token b),
                        (Value::Int8(a), Value::Int16(b)) => Value::Int16((a as i16) $op_token b),
                        (Value::Int8(a), Value::Int32(b)) => Value::Int32((a as i32) $op_token b),
                        (Value::Int8(a), Value::Int64(b)) => Value::Int64((a as i64) $op_token b),
                        (Value::Int8(a), Value::Int128(b)) => Value::Int128((a as i128) $op_token b),

                        (Value::Int16(a), Value::Int8(b)) => Value::Int16(a $op_token b as i16),
                        (Value::Int16(a), Value::Int16(b)) => Value::Int16(a $op_token b),
                        (Value::Int16(a), Value::Int32(b)) => Value::Int32((a as i32) $op_token b),
                        (Value::Int16(a), Value::Int64(b)) => Value::Int64((a as i64) $op_token b),
                        (Value::Int16(a), Value::Int128(b)) => Value::Int128((a as i128) $op_token b),

                        (Value::Int32(a), Value::Int8(b)) => Value::Int32(a $op_token b as i32),
                        (Value::Int32(a), Value::Int16(b)) => Value::Int32(a $op_token b as i32),
                        (Value::Int32(a), Value::Int32(b)) => Value::Int32(a $op_token b),
                        (Value::Int32(a), Value::Int64(b)) => Value::Int64((a as i64) $op_token b),
                        (Value::Int32(a), Value::Int128(b)) => Value::Int128((a as i128) $op_token b),

                        (Value::Int64(a), Value::Int8(b)) => Value::Int64(a $op_token b as i64),
                        (Value::Int64(a), Value::Int16(b)) => Value::Int64(a $op_token b as i64),
                        (Value::Int64(a), Value::Int32(b)) => Value::Int64(a $op_token b as i64),
                        (Value::Int64(a), Value::Int64(b)) => Value::Int64(a $op_token b),
                        (Value::Int64(a), Value::Int128(b)) => Value::Int128((a as i128) $op_token b),

                        (Value::Int128(a), Value::Int8(b)) => Value::Int128(a $op_token b as i128),
                        (Value::Int128(a), Value::Int16(b)) => Value::Int128(a $op_token b as i128),
                        (Value::Int128(a), Value::Int32(b)) => Value::Int128(a $op_token b as i128),
                        (Value::Int128(a), Value::Int64(b)) => Value::Int128(a $op_token b as i128),
                        (Value::Int128(a), Value::Int128(b)) => Value::Int128(a $op_token b),
                    }
                }
            )*
        }
    };
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Value {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
}

impl Value {
    /// Converts the Value to a usize, if possible.
    pub fn to_usize(&self) -> usize {
        match *self {
            Value::Int8(val) => val as usize,
            Value::Int16(val) => val as usize,
            Value::Int32(val) => val as usize,
            Value::Int64(val) => val as usize,
            Value::Int128(val) => val as usize, // Note: Potential loss of data if val is too large
        }
    }

    /// Converts the Value to a bool.
    /// Any non-zero value is considered true, and zero is considered false.
    pub fn to_bool(&self) -> bool {
        match *self {
            Value::Int8(val) => val != 0,
            Value::Int16(val) => val != 0,
            Value::Int32(val) => val != 0,
            Value::Int64(val) => val != 0,
            Value::Int128(val) => val != 0,
        }
    }
}

impl Value {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Value::Int8(val) => vec![0, *val as u8],
            Value::Int16(val) => {
                let mut bytes = vec![1];
                bytes.extend(&val.to_le_bytes());
                bytes
            },
            Value::Int32(val) => {
                let mut bytes = vec![2];
                bytes.extend(&val.to_le_bytes());
                bytes
            },
            Value::Int64(val) => {
                let mut bytes = vec![3];
                bytes.extend(&val.to_le_bytes());
                bytes
            },
            Value::Int128(val) => {
                let mut bytes = vec![4];
                bytes.extend(&val.to_le_bytes());
                bytes
            },
        }
    }
}

impl Value {
    fn from_bytes(bytes: &[u8]) -> (Self, usize) { // Returns Value and bytes consumed
        match bytes[0] {
            0 => (Value::Int8(bytes[1] as i8), 2),
            1 => {
                let val = i16::from_le_bytes([bytes[1], bytes[2]]);
                (Value::Int16(val), 3)
            },
            2 => {
                let val = i32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
                (Value::Int32(val), 5)
            },
            3 => {
                let val = i64::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8]]);
                (Value::Int64(val), 9)
            },
            4 => {
                let val = i128::from_le_bytes([
                    bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8],
                    bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15], bytes[16],
                ]);
                (Value::Int128(val), 17)
            },
            // Handle other Value variants...
            _ => unimplemented!(),
        }
    }
}

impl_ops! {
    add, add_method, +;
    sub, sub_method, -;
    mul, mul_method, *;
    div, div_method, /;
    and, and_method, &;
    or, or_method, |;
    xor, xor_method, ^;
    shr, shr_method, >>;
    shl, shl_method, <<
}

pub fn serialize(program: &[OpCode]) -> Vec<u8> {
    program.iter().flat_map(|op| op.to_bytes()).collect()
}

pub fn deserialize(bytes: &[u8]) -> Vec<OpCode> {
    let mut ops = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let (op, size) = OpCode::from_bytes(&bytes[i..]);
        ops.push(op);
        i += size;
    }
    ops
}

pub struct VM {
    pub stack: Vec<Value>,
    pub ip: usize,
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: Vec::new(),
            ip: 0,
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack underflow")
    }

    pub fn execute(&mut self, code: &[OpCode]) {
        self.ip = 0; // Initialize IP at the start of execution
        while self.ip < code.len() {
            match code[self.ip] {
                OpCode::Push(value) => {
                    self.stack.push(value);
                },
                OpCode::Jmp => {
                    let target = self.pop().to_usize(); // Assuming a method to convert Value to usize
                    self.ip = target; // Set IP to target, adjusting for 0-based indexing if necessary
                    continue;
                }
                OpCode::JmpIf => {
                    let condition = self.pop().to_bool(); // Assuming a method to convert Value to bool
                    let target = self.pop().to_usize();
                    if condition {
                        self.ip = target;
                        continue;
                    }
                }
                OpCode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.add(b));
                }
                OpCode::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.sub(b));
                }
                OpCode::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.mul(b));
                }
                OpCode::Div => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.div(b));
                }
                OpCode::And => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.and(b));
                }
                OpCode::Or => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.or(b));
                }
                OpCode::Xor => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.xor(b));
                }
                OpCode::Eq => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Int8((a == b) as i8));
                }
                OpCode::Neq => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Int8((a != b) as i8));
                }
                OpCode::Lt => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Int8((a < b) as i8));
                }
                OpCode::Lte => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Int8((a <= b) as i8));
                }
                OpCode::Gt => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Int8((a > b) as i8));
                }
                OpCode::Gte => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Int8((a >= b) as i8));
                }
                OpCode::Min => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.min(b));
                }
                OpCode::Max => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.max(b));
                }
                OpCode::Mux => {
                    let c = self.pop();
                    let b = self.pop();
                    let a = self.pop();
                    self.push(self.mux(a, b, c));
                }
                OpCode::ShiftRight => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.shr(b));
                }
                OpCode::ShiftLeft => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.shl(b));
                }
                OpCode::Dup => {
                    let value = *self.stack.last().expect("Stack underflow on Dup");
                    self.stack.push(value);
                }
                OpCode::NoOp => {
                    // Do nothing
                },

            }
            self.ip += 1; // Move to the next instruction unless jumped
        }
    }

    fn mux(&self, a: Value, b: Value, c: Value) -> Value {
        match a {
            Value::Int8(cond) => {
                if cond != 0 {
                    b
                } else {
                    c
                }
            }
            Value::Int16(cond) => {
                if cond != 0 {
                    b
                } else {
                    c
                }
            }
            Value::Int32(cond) => {
                if cond != 0 {
                    b
                } else {
                    c
                }
            }
            Value::Int64(cond) => {
                if cond != 0 {
                    b
                } else {
                    c
                }
            }
            Value::Int128(cond) => {
                if cond != 0 {
                    b
                } else {
                    c
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_i8_i16_promotion() {
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Int8(10)),
            OpCode::Push(Value::Int16(20)),
            OpCode::Add, 
        ];
        vm.execute(&bytecode);
        assert_eq!(vm.pop(), Value::Int16(30));
    }

    // Tests for Add operation
    #[test]
    fn test_add_i8_i8() {
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Int8(10)),
            OpCode::Push(Value::Int8(20)),
            OpCode::Add, 
        ];
        vm.execute(&bytecode);
        assert_eq!(vm.pop(), Value::Int8(30));
    }

    #[test]
    fn test_add_i16_i16() {
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Int16(300)),
            OpCode::Push(Value::Int16(500)),
            OpCode::Add, 
        ];
        vm.execute(&bytecode);
        assert_eq!(vm.pop(), Value::Int16(800));
    }

    #[test]
    fn test_add_i32_i32() {
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Int32(20000)),
            OpCode::Push(Value::Int32(30000)),
            OpCode::Add, 
        ];
        vm.execute(&bytecode);
        assert_eq!(vm.pop(), Value::Int32(50000));
    }

    // Testing type promotion
    #[test]
    fn test_add_i8_i32_promotion() {
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Int8(100)),
            OpCode::Push(Value::Int32(20000)),
            OpCode::Add, 
        ];
        vm.execute(&bytecode);
        assert_eq!(vm.pop(), Value::Int32(20100));
    }

    // Tests for Sub operation
    #[test]
    fn test_sub_i8_i8() {
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Int8(10)),
            OpCode::Push(Value::Int8(5)),
            OpCode::Sub, 
        ];
        vm.execute(&bytecode);
        assert_eq!(vm.pop(), Value::Int8(5));
    }

    #[test]
    fn test_sub_i16_i16() {
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Int16(500)),
            OpCode::Push(Value::Int16(300)),
            OpCode::Sub, 
        ];
        vm.execute(&bytecode);
        assert_eq!(vm.pop(), Value::Int16(200));
    }

    #[test]
    fn test_sub_i32_i32() {
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Int32(30000)),
            OpCode::Push(Value::Int32(20000)),
            OpCode::Sub, 
        ];
        vm.execute(&bytecode);
        assert_eq!(vm.pop(), Value::Int32(10000));
    }

    // Tests for Mux operation
    #[test]
    fn test_mux() {
        // Test setup
        let mut vm = VM::new();

        // Test case 1: Condition is true (non-zero), expect b to be selected
        let bytecode = [
            OpCode::Push(Value::Int8(1)),
            OpCode::Push(Value::Int8(10)),
            OpCode::Push(Value::Int8(20)),
            OpCode::Mux, 
        ];
        vm.execute(&bytecode);
        assert_eq!(
            vm.pop(),
            Value::Int8(10),
            "MUX did not select the correct value for true condition"
        );

        // Clear the stack for the next test
        vm.stack.clear();

        // Test case 2: Condition is false (zero), expect c to be selected
        vm.push(Value::Int8(0)); // Condition a, zero -> false
        vm.push(Value::Int8(10)); // Value b
        vm.push(Value::Int8(20)); // Value c
        let bytecode = [
            OpCode::Push(Value::Int8(0)),
            OpCode::Push(Value::Int8(10)),
            OpCode::Push(Value::Int8(20)),
            OpCode::Mux, 
        ];
        vm.execute(&bytecode);
        assert_eq!(
            vm.pop(),
            Value::Int8(20),
            "MUX did not select the correct value for false condition"
        );
    }

    #[test]
    fn test_complex_operation_with_mux() {
        let mut vm = VM::new();

        // Variables setup: a = 10, b = 5, c = 2
        vm.push(Value::Int32(2)); // c
        vm.push(Value::Int32(5)); // b
        vm.push(Value::Int32(10)); // a

        // Perform 'a > b' and leave the result on the stack
        vm.push(Value::Int32(10)); // Push 'a' again for comparison
        vm.push(Value::Int32(5)); // Push 'b' again for comparison
        vm.execute(&[OpCode::Gt]); // 'a > b' comparison

        // Path 1: Calculate (a + b) * c
        vm.push(Value::Int32(10)); // Push 'a' again
        vm.push(Value::Int32(5)); // Push 'b' again
        vm.execute(&[OpCode::Add]); // a + b
        vm.push(Value::Int32(2)); // Push 'c' again
        vm.execute(&[OpCode::Mul]); // (a + b) * c

        // Path 2: Calculate a - b (simpler operation for the 'else' path)
        vm.push(Value::Int32(10)); // Push 'a' again
        vm.push(Value::Int32(5)); // Push 'b' again
        vm.execute(&[OpCode::Sub]); // a - b

        // Execute Mux to select based on 'a > b'
        vm.execute(&[OpCode::Mux]);

        // Since 'a > b' is true, we expect the result of (a + b) * c => (10 + 5) * 2 = 30
        assert_eq!(vm.pop(), Value::Int32(30));
    }

    #[test]
    fn test_jmp() {
        let mut vm = VM::new();

        // Setup: Push a value, jump over an operation that would change it, and verify it remains unchanged.
        let bytecode = [
            OpCode::Push(Value::Int32(1)),
            OpCode::Push(Value::Int32(4)),
            OpCode::Jmp, // Jump to the instruction at index 2 (effectively the end).
            OpCode::Add, // This operation should be skipped due to the jump.
        ];

        vm.execute(&bytecode);

        // Verify: The value on the stack should be 1 since the add operation was skipped.
        assert_eq!(vm.pop(), Value::Int32(1));
    }

    #[test]
    fn test_jmpif_true_condition() {
        let mut vm = VM::new();

        // Setup: Push a true condition, and target index, perform a conditional jump, and an operation that should be skipped.
        let bytecode = [
            OpCode::Push(Value::Int32(4)),
            OpCode::Push(Value::Int32(1)),
            OpCode::JmpIf, // Conditional jump to the instruction at index 3 if the condition is true.
            OpCode::Add,   // This operation should be skipped.
        ];

        vm.execute(&bytecode);

        // The stack should be empty since both values are popped for the JmpIf operation and the jump is taken.
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_jmpif_false_condition() {
        let mut vm = VM::new();

        let bytecode = [
            OpCode::Push(Value::Int32(1)), // Another value to add.
            OpCode::Push(Value::Int32(19)), // Value to add if the jump is not taken.
            OpCode::Push(Value::Int32(7)), // Target index for the jump 
            OpCode::Push(Value::Int32(0)), // False condition (zero value).
            OpCode::JmpIf, // Should not jump because condition is false. Expects condition and target index on the stack.
            OpCode::Add,   // Should execute because the jump is not taken.
            OpCode::NoOp, // No-op to ensure there's an instruction at index 2 for the jump target
                           // OpCode::... (Ensure there's a valid opcode or no-op here if the jump target index is 2)
        ];

        vm.execute(&bytecode);

        // Verify: The addition operation should have occurred, resulting in 20.
        assert_eq!(vm.pop(), Value::Int32(20));
    }

    #[test]
    fn test_program_serialization_deserialization() {
        // Define a sample program with a mix of opcodes, including a push with a value
        let original_program = vec![
            OpCode::Push(Value::Int128(42)), // Sample value
            OpCode::Add,
            OpCode::Sub,
            OpCode::JmpIf,
            OpCode::Dup,
            OpCode::NoOp,
        ];

        // Serialize the program into bytes
        let serialized = serialize(&original_program);

        println!("Serialized program: {:?}", hex::encode(serialized.clone()));

        // Deserialize the bytes back into opcodes
        let deserialized_program = deserialize(&serialized);

        // Verify that the deserialized program matches the original
        assert_eq!(deserialized_program, original_program);
    }
}
