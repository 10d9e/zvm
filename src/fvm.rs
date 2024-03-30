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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
}

impl Value {
    fn promote_to_i128(self) -> i128 {
        match self {
            Value::Int8(val) => val as i128,
            Value::Int16(val) => val as i128,
            Value::Int32(val) => val as i128,
            Value::Int64(val) => val as i128,
            Value::Int128(val) => val,
        }
    }
}

pub struct VM {
    stack: Vec<Value>,
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
                OpCode::Add => self.binary_op(|a, b| Value::Int128(a + b)),
                OpCode::Sub => self.binary_op(|a, b| Value::Int128(a - b)),
                OpCode::Mul => self.binary_op(|a, b| Value::Int128(a * b)),
                OpCode::And => self.binary_op(|a, b| Value::Int128((a & b) as i128)),
                OpCode::Or => self.binary_op(|a, b| Value::Int128((a | b) as i128)),
                OpCode::Xor => self.binary_op(|a, b| Value::Int128((a ^ b) as i128)),
                OpCode::Eq => self.binary_op(|a, b| Value::Int128((a == b) as i128)),
                OpCode::Neq => self.binary_op(|a, b| Value::Int128((a != b) as i128)),
                OpCode::Lt => self.binary_op(|a, b| Value::Int128((a < b) as i128)),
                OpCode::Lte => self.binary_op(|a, b| Value::Int128((a <= b) as i128)),
                OpCode::Gt => self.binary_op(|a, b| Value::Int128((a > b) as i128)),
                OpCode::Gte => self.binary_op(|a, b| Value::Int128((a >= b) as i128)),
                OpCode::Min => self.binary_op(|a, b| Value::Int128(a.min(b))),
                OpCode::Max => self.binary_op(|a, b| Value::Int128(a.max(b))),
            }
        }
    }

    fn binary_op<F>(&mut self, op: F)
    where
        F: FnOnce(i128, i128) -> Value,
    {
        let b = self.pop().promote_to_i128();
        let a = self.pop().promote_to_i128();
        let result = op(a, b);
        self.push(result);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let mut vm = VM::new();
        vm.push(Value::Int8(5));
        vm.push(Value::Int8(3));
        vm.execute(vec![OpCode::Add]);
        assert_eq!(vm.pop(), Value::Int128(8));
    }

    #[test]
    fn test_subtraction() {
        let mut vm = VM::new();
        vm.push(Value::Int8(10));
        vm.push(Value::Int8(4));
        vm.execute(vec![OpCode::Sub]);
        assert_eq!(vm.pop(), Value::Int128(6));
    }

    #[test]
    fn test_multiplication() {
        let mut vm = VM::new();
        vm.push(Value::Int8(7));
        vm.push(Value::Int8(6));
        vm.execute(vec![OpCode::Mul]);
        assert_eq!(vm.pop(), Value::Int128(42));
    }

    #[test]
    fn test_and() {
        let mut vm = VM::new();
        vm.push(Value::Int8(0b1010));
        vm.push(Value::Int8(0b1100));
        vm.execute(vec![OpCode::And]);
        assert_eq!(vm.pop(), Value::Int128(0b1000));
    }

    #[test]
    fn test_or() {
        let mut vm = VM::new();
        vm.push(Value::Int8(0b1010));
        vm.push(Value::Int8(0b1100));
        vm.execute(vec![OpCode::Or]);
        assert_eq!(vm.pop(), Value::Int128(0b1110));
    }

    #[test]
    fn test_xor() {
        let mut vm = VM::new();
        vm.push(Value::Int8(0b1010));
        vm.push(Value::Int8(0b1100));
        vm.execute(vec![OpCode::Xor]);
        assert_eq!(vm.pop(), Value::Int128(0b0110));
    }

    #[test]
    fn test_eq() {
        let mut vm = VM::new();
        vm.push(Value::Int8(5));
        vm.push(Value::Int8(5));
        vm.execute(vec![OpCode::Eq]);
        assert_eq!(vm.pop(), Value::Int128(1));
    }

    #[test]
    fn test_neq() {
        let mut vm = VM::new();
        vm.push(Value::Int8(5));
        vm.push(Value::Int8(3));
        vm.execute(vec![OpCode::Neq]);
        assert_eq!(vm.pop(), Value::Int128(1));
    }

    #[test]
    fn test_lt() {
        let mut vm = VM::new();
        vm.push(Value::Int8(3));
        vm.push(Value::Int8(5));
        vm.execute(vec![OpCode::Lt]);
        assert_eq!(vm.pop(), Value::Int128(1));
    }

    #[test]
    fn test_lte() {
        let mut vm = VM::new();
        vm.push(Value::Int8(5));
        vm.push(Value::Int8(5));
        vm.execute(vec![OpCode::Lte]);
        assert_eq!(vm.pop(), Value::Int128(1));
    }

    #[test]
    fn test_gt() {
        let mut vm = VM::new();
        vm.push(Value::Int8(6));
        vm.push(Value::Int8(5));
        vm.execute(vec![OpCode::Gt]);
        assert_eq!(vm.pop(), Value::Int128(1));
    }

    #[test]
    fn test_gte() {
        let mut vm = VM::new();
        vm.push(Value::Int8(5));
        vm.push(Value::Int8(5));
        vm.execute(vec![OpCode::Gte]);
        assert_eq!(vm.pop(), Value::Int128(1));
    }

    #[test]
    fn test_min() {
        let mut vm = VM::new();
        vm.push(Value::Int8(10));
        vm.push(Value::Int8(20));
        vm.execute(vec![OpCode::Min]);
        assert_eq!(vm.pop(), Value::Int128(10));
    }

    #[test]
    fn test_max() {
        let mut vm = VM::new();
        vm.push(Value::Int8(-5));
        vm.push(Value::Int8(15));
        vm.execute(vec![OpCode::Max]);
        assert_eq!(vm.pop(), Value::Int128(15));
    }
}
