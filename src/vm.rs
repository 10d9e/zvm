#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Add,
    Sub,
    Mul,
    And,
    Or,
    Xor,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    Min,
    Max,
    Mux,    
}

macro_rules! impl_ops {
    ($($op:ident, $op_method:ident, $op_token:tt);*) => {
        impl Value {
            $(
                fn $op(self, other: Self) -> Self {
                    match (self, other) {
                        (Value::Int8(a), Value::Int8(b)) => Value::Int8(a $op_token b),
                        (Value::Int8(a), Value::Int16(b)) => Value::Int16(a as i16 $op_token b),
                        (Value::Int8(a), Value::Int32(b)) => Value::Int32(a as i32 $op_token b),
                        (Value::Int8(a), Value::Int64(b)) => Value::Int64(a as i64 $op_token b),
                        (Value::Int8(a), Value::Int128(b)) => Value::Int128(a as i128 $op_token b),

                        (Value::Int16(a), Value::Int8(b)) => Value::Int16(a $op_token b as i16),
                        (Value::Int16(a), Value::Int16(b)) => Value::Int16(a $op_token b),
                        (Value::Int16(a), Value::Int32(b)) => Value::Int32(a as i32 $op_token b),
                        (Value::Int16(a), Value::Int64(b)) => Value::Int64(a as i64 $op_token b),
                        (Value::Int16(a), Value::Int128(b)) => Value::Int128(a as i128 $op_token b),

                        (Value::Int32(a), Value::Int8(b)) => Value::Int32(a $op_token b as i32),
                        (Value::Int32(a), Value::Int16(b)) => Value::Int32(a $op_token b as i32),
                        (Value::Int32(a), Value::Int32(b)) => Value::Int32(a $op_token b),
                        (Value::Int32(a), Value::Int64(b)) => Value::Int64(a as i64 $op_token b),
                        (Value::Int32(a), Value::Int128(b)) => Value::Int128(a as i128 $op_token b),

                        (Value::Int64(a), Value::Int8(b)) => Value::Int64(a $op_token b as i64),
                        (Value::Int64(a), Value::Int16(b)) => Value::Int64(a $op_token b as i64),
                        (Value::Int64(a), Value::Int32(b)) => Value::Int64(a $op_token b as i64),
                        (Value::Int64(a), Value::Int64(b)) => Value::Int64(a $op_token b),
                        (Value::Int64(a), Value::Int128(b)) => Value::Int128(a as i128 $op_token b),

                        (Value::Int128(a), Value::Int8(b)) => Value::Int128(a $op_token b as i128),
                        (Value::Int128(a), Value::Int16(b)) => Value::Int128(a $op_token b as i128),
                        (Value::Int128(a), Value::Int32(b)) => Value::Int128(a $op_token b as i128),
                        (Value::Int128(a), Value::Int64(b)) => Value::Int128(a $op_token b as i128),
                        (Value::Int128(a), Value::Int128(b)) => Value::Int128(a $op_token b),

                        // Implement other combinations and types as necessary
                        _ => unimplemented!("Operation not implemented for this type combination"),
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

impl_ops! {
    add, add_method, +;
    sub, sub_method, -;
    mul, mul_method, *;
    and, and_method, &;
    or, or_method, |;
    xor, xor_method, ^
}

pub struct VM {
    pub stack: Vec<Value>,
}

impl VM {
    pub fn new() -> VM {
        VM { stack: Vec::new() }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack underflow")
    }

    pub fn execute(&mut self, code: Vec<OpCode>) {
        for op in code {
            match op {
                OpCode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.add(b));
                },
                OpCode::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.sub(b));
                },
                OpCode::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.mul(b));
                },
                OpCode::And => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.and(b));
                },
                OpCode::Or => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.or(b));
                },
                OpCode::Xor => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.xor(b));
                },
                OpCode::Eq => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Int8((a == b) as i8));
                },
                OpCode::Neq => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Int8((a != b) as i8));
                },
                OpCode::Lt => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Int8((a < b) as i8));
                },
                OpCode::Lte => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Int8((a <= b) as i8));
                },
                OpCode::Gt => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Int8((a > b) as i8));
                },
                OpCode::Gte => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Int8((a >= b) as i8));
                },
                OpCode::Min => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.min(b));
                },
                OpCode::Max => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.max(b));
                },
                OpCode::Mux => {
                    let c = self.pop();
                    let b = self.pop();
                    let a = self.pop();
                    self.push(self.mux(a, b, c));
                },
            }
        }
    }

    fn mux(&self, a: Value, b: Value, c: Value) -> Value {
        match a {
            Value::Int8(cond) => if cond != 0 { b } else { c },
            Value::Int16(cond) => if cond != 0 { b } else { c },
            Value::Int32(cond) => if cond != 0 { b } else { c },
            Value::Int64(cond) => if cond != 0 { b } else { c },
            Value::Int128(cond) => if cond != 0 { b } else { c },
            _ => unimplemented!("Mux operation not supported for this type"),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_i8_i16_promotion() {
        let mut vm = VM::new();
        vm.push(Value::Int8(10));
        vm.push(Value::Int16(20));
        vm.execute(vec![OpCode::Add]);
        assert_eq!(vm.pop(), Value::Int16(30));
    }

    // Tests for Add operation
    #[test]
    fn test_add_i8_i8() {
        let mut vm = VM::new();
        vm.push(Value::Int8(10));
        vm.push(Value::Int8(20));
        vm.execute(vec![OpCode::Add]);
        assert_eq!(vm.pop(), Value::Int8(30));
    }

    #[test]
    fn test_add_i16_i16() {
        let mut vm = VM::new();
        vm.push(Value::Int16(300));
        vm.push(Value::Int16(500));
        vm.execute(vec![OpCode::Add]);
        assert_eq!(vm.pop(), Value::Int16(800));
    }

    #[test]
    fn test_add_i32_i32() {
        let mut vm = VM::new();
        vm.push(Value::Int32(20000));
        vm.push(Value::Int32(30000));
        vm.execute(vec![OpCode::Add]);
        assert_eq!(vm.pop(), Value::Int32(50000));
    }

    // Testing type promotion
    #[test]
    fn test_add_i8_i32_promotion() {
        let mut vm = VM::new();
        vm.push(Value::Int8(100));
        vm.push(Value::Int32(20000));
        vm.execute(vec![OpCode::Add]);
        assert_eq!(vm.pop(), Value::Int32(20100));
    }

    // Tests for Sub operation
    #[test]
    fn test_sub_i8_i8() {
        let mut vm = VM::new();
        vm.push(Value::Int8(10));
        vm.push(Value::Int8(5));
        vm.execute(vec![OpCode::Sub]);
        assert_eq!(vm.pop(), Value::Int8(5));
    }

    #[test]
    fn test_sub_i16_i16() {
        let mut vm = VM::new();
        vm.push(Value::Int16(500));
        vm.push(Value::Int16(300));
        vm.execute(vec![OpCode::Sub]);
        assert_eq!(vm.pop(), Value::Int16(200));
    }

    #[test]
    fn test_sub_i32_i32() {
        let mut vm = VM::new();
        vm.push(Value::Int32(30000));
        vm.push(Value::Int32(20000));
        vm.execute(vec![OpCode::Sub]);
        assert_eq!(vm.pop(), Value::Int32(10000));
    }

    // Tests for Mux operation
    #[test]
    fn test_mux() {
        // Test setup
        let mut vm = VM::new();

        // Test case 1: Condition is true (non-zero), expect b to be selected
        vm.push(Value::Int8(1)); // Condition a, non-zero -> true
        vm.push(Value::Int8(10)); // Value b
        vm.push(Value::Int8(20)); // Value c
        vm.execute(vec![OpCode::Mux]);
        assert_eq!(vm.pop(), Value::Int8(10), "MUX did not select the correct value for true condition");

        // Clear the stack for the next test
        vm.stack.clear();

        // Test case 2: Condition is false (zero), expect c to be selected
        vm.push(Value::Int8(0)); // Condition a, zero -> false
        vm.push(Value::Int8(10)); // Value b
        vm.push(Value::Int8(20)); // Value c
        vm.execute(vec![OpCode::Mux]);
        assert_eq!(vm.pop(), Value::Int8(20), "MUX did not select the correct value for false condition");
    }


}
