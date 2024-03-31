use tfhe::{prelude::*, FheBool, FheInt32Id};
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32, FheUint8, FheUint16, FheUint64, FheUint128};
use tfhe::FheUint;

use std::ops::Add;

#[derive(Clone)]
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
    /*
    Jmp(i32),   // Jump to an instruction index unconditionally
    JmpIf(i32), // Jump if the top of the stack is nonzero (true)
    */

    Push(Value), // Push now carries a Value with it
    Dup,         // Duplicate the top item on the stack
    NoOp,        // No operation

    Inc,
    Dec,
    Load(i32), // Assuming address space is indexed by i32
    Store(i32),
    Swap,
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
            /*
            OpCode::Jmp(address) => {
                let mut bytes = vec![18];
                bytes.extend(address.to_le_bytes());
                bytes
            }
            OpCode::JmpIf(address) => {
                let mut bytes = vec![19];
                bytes.extend(address.to_le_bytes());
                bytes
            }
             */
            OpCode::NoOp => vec![20],
            OpCode::Dup => vec![21],
            OpCode::Push(value) => {
                let mut bytes = vec![22]; // Example byte for Push
                bytes.extend(value.to_bytes());
                bytes
            }
            OpCode::Inc => vec![23],
            OpCode::Dec => vec![24],
            OpCode::Load(address) => {
                let mut bytes = vec![25];
                bytes.extend(address.to_le_bytes());
                bytes
            }
            OpCode::Store(address) => {
                let mut bytes = vec![26];
                bytes.extend(address.to_le_bytes());
                bytes
            }
            OpCode::Swap => vec![27],
        }
    }
}

impl OpCode {
    fn from_bytes(bytes: &[u8]) -> (Self, usize) {
        // Returns OpCode and bytes consumed
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
            /* 
            18 => {
                let address = i32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
                (OpCode::Jmp(address), 5)
            }
            19 => {
                let address = i32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
                (OpCode::JmpIf(address), 5)
            }
            */
            20 => (OpCode::NoOp, 1),
            21 => (OpCode::Dup, 1),
            22 => {
                let (value, size) = Value::from_bytes(&bytes[1..]);
                (OpCode::Push(value), size + 1)
            }
            23 => (OpCode::Inc, 1),
            24 => (OpCode::Dec, 1),
            25 => {
                let address = i32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
                (OpCode::Load(address), 5)
            }
            26 => {
                let address = i32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
                (OpCode::Store(address), 5)
            }
            27 => (OpCode::Swap, 1),
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
                        (Value::Int8(a), Value::Int16(b)) => Value::Int16(FheUint16::cast_from(a) $op_token b),
                        (Value::Int8(a), Value::Int32(b)) => Value::Int32(FheUint32::cast_from(a) $op_token b),
                        (Value::Int8(a), Value::Int64(b)) => Value::Int64(FheUint64::cast_from(a) $op_token b),
                        (Value::Int8(a), Value::Int128(b)) => Value::Int128(FheUint128::cast_from(a) $op_token b),

                        (Value::Int16(a), Value::Int8(b)) => Value::Int16(a $op_token FheUint16::cast_from(b)),
                        (Value::Int16(a), Value::Int16(b)) => Value::Int16(a $op_token b),
                        (Value::Int16(a), Value::Int32(b)) => Value::Int32(FheUint32::cast_from(a) $op_token b),
                        (Value::Int16(a), Value::Int64(b)) => Value::Int64(FheUint64::cast_from(a) $op_token b),
                        (Value::Int16(a), Value::Int128(b)) => Value::Int128(FheUint128::cast_from(a) $op_token b),
                        
                        (Value::Int32(a), Value::Int8(b)) => Value::Int32(a $op_token FheUint32::cast_from(b)),
                        (Value::Int32(a), Value::Int16(b)) => Value::Int32(a $op_token FheUint32::cast_from(b)),
                        (Value::Int32(a), Value::Int32(b)) => Value::Int32(a $op_token b),
                        (Value::Int32(a), Value::Int64(b)) => Value::Int64(FheUint64::cast_from(a) $op_token b),
                        (Value::Int32(a), Value::Int128(b)) => Value::Int128(FheUint128::cast_from(a) $op_token b),

                        (Value::Int64(a), Value::Int8(b)) => Value::Int64(a $op_token FheUint64::cast_from(b)),
                        (Value::Int64(a), Value::Int16(b)) => Value::Int64(a $op_token FheUint64::cast_from(b)),
                        (Value::Int64(a), Value::Int32(b)) => Value::Int64(a $op_token FheUint64::cast_from(b)),
                        (Value::Int64(a), Value::Int64(b)) => Value::Int64(a $op_token b),
                        (Value::Int64(a), Value::Int128(b)) => Value::Int128(FheUint128::cast_from(a) $op_token b),
                        
                        (Value::Int128(a), Value::Int8(b)) => Value::Int128(a $op_token FheUint128::cast_from(b)),
                        (Value::Int128(a), Value::Int16(b)) => Value::Int128(a $op_token FheUint128::cast_from(b)),
                        (Value::Int128(a), Value::Int32(b)) => Value::Int128(a $op_token FheUint128::cast_from(b)),
                        (Value::Int128(a), Value::Int64(b)) => Value::Int128(a $op_token FheUint128::cast_from(b)),
                        (Value::Int128(a), Value::Int128(b)) => Value::Int128(a $op_token b),
                    }
                }
            )*
        }
    };
}

#[derive(Clone)]
pub enum BoolValue {
    FvmBool(FheBool),
}

impl FheEq<BoolValue> for BoolValue {
    fn eq(&self, other: BoolValue) -> FheBool {
        match (self, other) {
            (BoolValue::FvmBool(a), BoolValue::FvmBool(b)) => a.eq(b),
        }
    }

    fn ne(&self, other: BoolValue) -> FheBool {
        match (self, other) {
            (BoolValue::FvmBool(a), BoolValue::FvmBool(b)) => a.ne(b),
        }
    }
}

impl BoolValue {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            BoolValue::FvmBool(val) => {
                let mut bytes = vec![0];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            },
        }
    }

    fn from_bytes(bytes: &[u8]) -> (Self, usize) {
        // Returns Value and bytes consumed
        match bytes[0] {
            0 => {
                let val:FheBool = bincode::deserialize(&bytes[1..]).unwrap();
                (BoolValue::FvmBool(val), 3)
            }
            // Handle other Value variants...
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone)]
pub enum Value {
    Int8(FheUint8),
    Int16(FheUint16),
    Int32(FheUint32),
    Int64(FheUint64),
    Int128(FheUint128),
}

impl FheEq<Value> for Value {
    fn eq(&self, other: Self) -> FheBool {
        match (self, other) {
            (Value::Int8(a), Value::Int8(b)) => a.eq(b),
            (Value::Int16(a), Value::Int16(b)) => a.eq(b),
            (Value::Int32(a), Value::Int32(b)) => a.eq(b),
            (Value::Int64(a), Value::Int64(b)) => a.eq(b),
            (Value::Int128(a), Value::Int128(b)) => a.eq(b),
            _ => unimplemented!(),
        }
    }

    fn ne(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Int8(a), Value::Int8(b)) => a.ne(b),
            (Value::Int16(a), Value::Int16(b)) => a.ne(b),
            (Value::Int32(a), Value::Int32(b)) => a.ne(b),
            (Value::Int64(a), Value::Int64(b)) => a.ne(b),
            (Value::Int128(a), Value::Int128(b)) => a.ne(b),
            _ => unimplemented!(),
        }
    }
}

impl FheOrd<Value> for Value {
    fn lt(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Int8(a), Value::Int8(b)) => a.lt(b),
            (Value::Int16(a), Value::Int16(b)) => a.lt(b),
            (Value::Int32(a), Value::Int32(b)) => a.lt(b),
            (Value::Int64(a), Value::Int64(b)) => a.lt(b),
            (Value::Int128(a), Value::Int128(b)) => a.lt(b),
            _ => unimplemented!(),
        }
    }

    fn le(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Int8(a), Value::Int8(b)) => a.le(b),
            (Value::Int16(a), Value::Int16(b)) => a.le(b),
            (Value::Int32(a), Value::Int32(b)) => a.le(b),
            (Value::Int64(a), Value::Int64(b)) => a.le(b),
            (Value::Int128(a), Value::Int128(b)) => a.le(b),
            _ => unimplemented!(),
        }
    }

    fn gt(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Int8(a), Value::Int8(b)) => a.gt(b),
            (Value::Int16(a), Value::Int16(b)) => a.gt(b),
            (Value::Int32(a), Value::Int32(b)) => a.gt(b),
            (Value::Int64(a), Value::Int64(b)) => a.gt(b),
            (Value::Int128(a), Value::Int128(b)) => a.gt(b),
            _ => unimplemented!(),
        }
    }

    fn ge(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Int8(a), Value::Int8(b)) => a.ge(b),
            (Value::Int16(a), Value::Int16(b)) => a.ge(b),
            (Value::Int32(a), Value::Int32(b)) => a.ge(b),
            (Value::Int64(a), Value::Int64(b)) => a.ge(b),
            (Value::Int128(a), Value::Int128(b)) => a.ge(b),
            _ => unimplemented!(),
        }
    }
}

impl FheMin<Value> for Value {
    type Output = Self;

    fn min(&self, other: Value) -> Self {
        match (self, other) {
            (Value::Int8(a), Value::Int8(b)) => Value::Int8(a.min(b)),
            (Value::Int16(a), Value::Int16(b)) => Value::Int16(a.min(b)),
            (Value::Int32(a), Value::Int32(b)) => Value::Int32(a.min(b)),
            (Value::Int64(a), Value::Int64(b)) => Value::Int64(a.min(b)),
            (Value::Int128(a), Value::Int128(b)) => Value::Int128(a.min(b)),
            _ => unimplemented!(),
        }
    }
}

impl FheMax<Value> for Value {
    type Output = Self;

    fn max(&self, other: Value) -> Self {
        match (self, other) {
            (Value::Int8(a), Value::Int8(b)) => Value::Int8(a.max(b)),
            (Value::Int16(a), Value::Int16(b)) => Value::Int16(a.max(b)),
            (Value::Int32(a), Value::Int32(b)) => Value::Int32(a.max(b)),
            (Value::Int64(a), Value::Int64(b)) => Value::Int64(a.max(b)),
            (Value::Int128(a), Value::Int128(b)) => Value::Int128(a.max(b)),
            _ => unimplemented!(),
        }
    }
}

impl Value {
     /* 
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
    */

    /*
    /// Converts the Value to a bool.
    /// Any non-zero value is considered true, and zero is considered false.
    pub fn to_bool(&self) -> FheBool {
        match *self {
            Value::Int8(val) => val.try_into().unwrap() != 0,
            Value::Int16(val) => val != 0,
            Value::Int32(val) => val != 0,
            Value::Int64(val) => val != 0,
            Value::Int128(val) => val != 0,
        }
    }
     */
}

impl Value {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Value::Int8(val) => {
                let mut bytes = vec![1];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            },
            Value::Int16(val) => {
                let mut bytes = vec![2];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Int32(val) => {
                let mut bytes = vec![3];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Int64(val) => {
                let mut bytes = vec![4];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Int128(val) => {
                let mut bytes = vec![5];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
        }
    }

    fn from_bytes(bytes: &[u8]) -> (Self, usize) {
        // Returns Value and bytes consumed
        match bytes[0] {
            1 => {
                let val:FheUint8 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Int8(val), 3)
            }
            2 => {
                let val:FheUint16 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Int16(val), 3)
            }
            3 => {
                let val:FheUint32 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Int32(val), 3)
            }
            4 => {
                let val:FheUint64 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Int64(val), 3)
            }
            5 => {
                let val:FheUint128 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Int128(val), 3)
            }
            // Handle other Value variants...
            _ => unimplemented!(),
        }
    }
}

impl Add<u8> for Value {
    type Output = Self;

    fn add(self, other: u8) -> Self {
        match self {
            Value::Int8(val) => Value::Int8(val + other),
            Value::Int16(val) => Value::Int16(val + other as u16),
            Value::Int32(val) => Value::Int32(val + other as u32),
            Value::Int64(val) => Value::Int64(val + other as u64),
            Value::Int128(val) => Value::Int128(val + other as u128),
        }
    }
}

/* 
impl Add<Value> for Value {
    type Output = Self;

    fn add(self, other: Value) -> Self {
        match (self, other) {
            (Value::Int8(a), Value::Int8(b)) => Value::Int8(a + b),
            (Value::Int8(a), Value::Int16(b)) => Value::Int16(FheUint16::cast_from(a) + b),
            (Value::Int8(a), Value::Int32(b)) => Value::Int32(FheUint32::cast_from(a) + b),
            (Value::Int8(a), Value::Int64(b)) => Value::Int64(FheUint64::cast_from(a) + b),
            (Value::Int8(a), Value::Int128(b)) => Value::Int128(FheUint128::cast_from(a) + b),

            (Value::Int16(a), Value::Int8(b)) => Value::Int16(a + FheUint16::cast_from(b)),
            (Value::Int16(a), Value::Int16(b)) => Value::Int16(a + b),
            (Value::Int16(a), Value::Int32(b)) => Value::Int32(FheUint32::cast_from(a) + b),
            (Value::Int16(a), Value::Int64(b)) => Value::Int64(FheUint64::cast_from(a) + b),
            (Value::Int16(a), Value::Int128(b)) => Value::Int128(FheUint128::cast_from(a) + b),
            
            (Value::Int32(a), Value::Int8(b)) => Value::Int32(a + FheUint32::cast_from(b)),
            (Value::Int32(a), Value::Int16(b)) => Value::Int32(a + FheUint32::cast_from(b)),
            (Value::Int32(a), Value::Int32(b)) => Value::Int32(a + b),
            (Value::Int32(a), Value::Int64(b)) => Value::Int64(FheUint64::cast_from(a) + b),
            (Value::Int32(a), Value::Int128(b)) => Value::Int128(FheUint128::cast_from(a) + b),

            (Value::Int64(a), Value::Int8(b)) => Value::Int64(a + FheUint64::cast_from(b)),
            (Value::Int64(a), Value::Int16(b)) => Value::Int64(a + FheUint64::cast_from(b)),
            (Value::Int64(a), Value::Int32(b)) => Value::Int64(a + FheUint64::cast_from(b)),
            (Value::Int64(a), Value::Int64(b)) => Value::Int64(a + b),
            (Value::Int64(a), Value::Int128(b)) => Value::Int128(FheUint128::cast_from(a) + b),
            
            (Value::Int128(a), Value::Int8(b)) => Value::Int128(a + FheUint128::cast_from(b)),
            (Value::Int128(a), Value::Int16(b)) => Value::Int128(a + FheUint128::cast_from(b)),
            (Value::Int128(a), Value::Int32(b)) => Value::Int128(a + FheUint128::cast_from(b)),
            (Value::Int128(a), Value::Int64(b)) => Value::Int128(a + FheUint128::cast_from(b)),
            (Value::Int128(a), Value::Int128(b)) => Value::Int128(a + b),

            _ => unimplemented!(),
        }
    }
}
*/

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
    memory: Vec<Value>, // For Load and Store operations
    ip: usize,          // Instruction pointer
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: Vec::new(),
            ip: 0,
            memory: Vec::new(),
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
                }
                /*
                OpCode::Jmp(target) => {
                    self.ip = target.try_into().unwrap(); // Set IP to target, adjusting for 0-based indexing if necessary
                    continue;
                }
                OpCode::JmpIf(target) => {
                    let condition = self.pop().to_bool(); // Assuming a method to convert Value to bool
                    if condition {
                        self.ip = target.try_into().unwrap();
                        continue;
                    }
                }
                */
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
                    let val = Value::Int8(FheUint8::cast_from(a.eq(b)));
                    self.push(val);
                }
                OpCode::Neq => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Int8(FheUint8::cast_from(a.ne(b)));
                    self.push(val);
                }
                OpCode::Lt => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Int8(FheUint8::cast_from(a.lt(b)));
                    self.push(val);
                }
                OpCode::Lte => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Int8(FheUint8::cast_from(a.le(b)));
                    self.push(val);
                }
                OpCode::Gt => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Int8(FheUint8::cast_from(a.gt(b)));
                    self.push(val);
                }
                OpCode::Gte => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Int8(FheUint8::cast_from(a.ge(b)));
                    self.push(val);
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
                }
                OpCode::Inc => {
                    let a = self.pop();
                    self.push(a + 1u8);
                }
                OpCode::Dec => {
                    let a = self.pop();
                    self.push(a.sub(Value::Int8(1)));
                }
                OpCode::Load(address) => {
                    // Assume memory is a Vec<Value>, and address is within bounds
                    let uaddress: usize = address.try_into().unwrap();
                    let value = self.memory[uaddress];
                    self.stack.push(value);
                }
                OpCode::Store(address) => {
                    let value = self.pop();
                    // Ensure memory is large enough to handle address
                    let uaddress: usize = address.try_into().unwrap();
                    if self.memory.len() <= uaddress {
                        self.memory.resize(uaddress + 1, Value::Int32(0)); // Example resizing with default value
                    }
                    self.memory[uaddress] = value;
                }
                OpCode::Swap => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a);
                    self.push(b);
                }
            }
            self.ip += 1; // Move to the next instruction unless jumped
        }
    }

    fn mux(&self, a: Value, b: Value, c: Value) -> Value {
        match a {
            Value::Int8(cond) => {
                cond.cmux(b, c)
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
    fn test_jmp_false() {
        let mut vm = VM::new();

        // Setup: Push a value, jump over an operation that would change it, and verify it remains unchanged.
        let bytecode = [
            OpCode::Push(Value::Int32(0)),
            OpCode::Jmp(5), // Jump to the instruction at index 2 (effectively the end).
            OpCode::Push(Value::Int32(42)),
            OpCode::Push(Value::Int32(43)),
            OpCode::Add,  // This operation should be skipped due to the jump.
            OpCode::NoOp, // No-op to ensure there's an instruction at index 2 for the jump target
        ];

        vm.execute(&bytecode);

        // Verify: The value on the stack should be 1 since the add operation was skipped.
        assert_eq!(vm.pop(), Value::Int32(0));
    }

    #[test]
    fn test_jmpif_true_condition() {
        let mut vm = VM::new();

        // Setup: Push a true condition, and target index, perform a conditional jump, and an operation that should be skipped.
        let bytecode = [
            OpCode::Push(Value::Int32(1)),
            OpCode::JmpIf(3), // Conditional jump to the instruction at index 3 if the condition is true.
            OpCode::Add,      // This operation should be skipped.
        ];

        vm.execute(&bytecode);

        // The stack should be empty since both values are popped for the JmpIf operation and the jump is taken.
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_jmpif_true_condition_2() {
        let mut vm = VM::new();

        // Setup: Push a true condition, and target index, perform a conditional jump, and an operation that should be skipped.
        let bytecode = [
            OpCode::Push(Value::Int8(1)),
            OpCode::JmpIf(3), // Conditional jump to the instruction at index 3 if the condition is true.
            OpCode::NoOp,     // No-op to ensure there's an instruction at
            OpCode::Push(Value::Int128(10000000000000000000000000000000000000)),
            OpCode::Push(Value::Int8(42)),
            OpCode::Add, // This operation should be skipped.
        ];

        vm.execute(&bytecode);

        // Verify: The addition operation should have occurred, resulting in 20.
        assert_eq!(
            vm.pop(),
            Value::Int128(10000000000000000000000000000000000042)
        );
    }

    #[test]
    fn test_jmpif_false_condition() {
        let mut vm = VM::new();

        let bytecode = [
            OpCode::Push(Value::Int32(1)),  // Another value to add.
            OpCode::Push(Value::Int32(19)), // Value to add if the jump is not taken.
            OpCode::Push(Value::Int32(0)),  // False condition (zero value).
            OpCode::JmpIf(7), // Should not jump because condition is false. Expects condition and target index on the stack.
            OpCode::Add,      // Should execute because the jump is not taken.
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
        let original_program = [
            OpCode::Add,
            OpCode::Sub,
            OpCode::Mul,
            OpCode::Div,
            OpCode::And,
            OpCode::Or,
            OpCode::Xor,
            OpCode::ShiftRight,
            OpCode::ShiftLeft,
            OpCode::Eq,
            OpCode::Neq,
            OpCode::Lt,
            OpCode::Gt,
            OpCode::Gte,
            OpCode::Min,
            OpCode::Max,
            OpCode::Mux,
            OpCode::Jmp(42),
            OpCode::JmpIf(43),
            OpCode::Push(Value::Int8(8)),
            OpCode::Push(Value::Int16(16)),
            OpCode::Push(Value::Int32(32)),
            OpCode::Push(Value::Int64(64)),
            OpCode::Push(Value::Int128(128)),
            OpCode::Dup,
            OpCode::NoOp,
            OpCode::Inc,
            OpCode::Dec,
            OpCode::Load(77),
            OpCode::Store(88),
            OpCode::Swap,
            // OpCode::Const(99),
        ];

        // Serialize the program into bytes
        let serialized = serialize(&original_program);

        println!("Serialized program: {:?}", hex::encode(serialized.clone()));

        // Deserialize the bytes back into opcodes
        let deserialized_program = deserialize(&serialized);

        // Verify that the deserialized program matches the original
        assert_eq!(deserialized_program, original_program);
    }

    #[test]
    fn test_inc() {
        let mut vm = VM::new();
        vm.push(Value::Int32(10));
        vm.execute(&[OpCode::Inc]);
        assert_eq!(vm.pop(), Value::Int32(11));
    }

    #[test]
    fn test_dec() {
        let mut vm = VM::new();
        vm.push(Value::Int32(10));
        vm.execute(&[OpCode::Dec]);
        assert_eq!(vm.pop(), Value::Int32(9));
    }

    #[test]
    fn test_load_store() {
        let mut vm = VM::new();
        vm.memory.resize(1, Value::Int32(0)); // Ensure memory is initialized
        vm.push(Value::Int32(42)); // Value to store
        vm.execute(&[OpCode::Store(0)]); // Store at address 0
        vm.execute(&[OpCode::Load(0)]); // Load from address 0
        assert_eq!(vm.pop(), Value::Int32(42));
    }

    #[test]
    fn test_swap() {
        let mut vm = VM::new();
        vm.push(Value::Int32(10));
        vm.push(Value::Int32(20));
        vm.execute(&[OpCode::Swap]);
        assert_eq!(vm.pop(), Value::Int32(10));
        assert_eq!(vm.pop(), Value::Int32(20));
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_inc_overflow() {
        let mut vm = VM::new();
        vm.push(Value::Int32(i32::MAX));
        vm.execute(&[OpCode::Inc]);
        //assert_eq!(vm.pop(), Value::Int32(i32::MIN));
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_dec_underflow() {
        let mut vm = VM::new();
        vm.push(Value::Int32(i32::MIN));
        vm.execute(&[OpCode::Dec]);
        //assert_eq!(vm.pop(), Value::Int32(i32::MAX));
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 100")]
    fn test_load_uninitialized_memory() {
        let mut vm = VM::new();
        // Assuming memory is automatically initialized to a default value when accessed
        vm.execute(&[OpCode::Load(100)]); // Load from an "uninitialized" address
        assert_eq!(vm.pop(), Value::Int32(0)); // Adjust based on your VM's default memory initialization
    }

    #[test]
    fn test_store_and_load() {
        let mut vm = VM::new();
        vm.push(Value::Int32(42));
        vm.execute(&[OpCode::Store(0)]); // Store at address 0
        vm.execute(&[OpCode::Load(0)]); // Load from address 0
        assert_eq!(vm.pop(), Value::Int32(42));
    }

    // Test Swap with insufficient stack depth

    #[test]
    #[should_panic(expected = "Stack underflow")]
    fn test_swap_insufficient_stack() {
        let mut vm = VM::new();
        vm.push(Value::Int32(10));
        // No second value to swap with, testing error handling or stack underflow management
        vm.execute(&[OpCode::Swap]);
        // Outcome depends on VM's error handling strategy (e.g., exception, error state, or stack underflow handling)
        // This assertion might need adjustment based on how your VM handles such scenarios
        assert_eq!(vm.stack.len(), 1); // For example, checking stack size remains unchanged
    }

    #[test]
    fn test_fibonacci_sequence() {
        let mut vm = VM::new();
        // Assuming the VM has been appropriately initialized with memory and stack.

        // Simulated program to calculate the 5th Fibonacci number
        // This assumes a somewhat "idealized" opcode layout and may need adjustments
        let program = vec![
            OpCode::Push(Value::Int32(5)), // n = 5, calculate the 5th Fibonacci number
            OpCode::Push(Value::Int32(0)), // Fibonacci[0]
            OpCode::Push(Value::Int32(1)), // Fibonacci[1]
            OpCode::Push(Value::Int32(1)), // Counter for how many Fibonacci numbers have been calculated
            // Start of the loop to calculate Fibonacci numbers
            // Loop condition: if counter < n, calculate the next Fibonacci number
            OpCode::Dup,                   // Duplicate the counter
            OpCode::Push(Value::Int32(5)), // Push n (5) to stack for comparison
            OpCode::Lt,                    // Compare if counter < n
            OpCode::JmpIf(8), // Jump to the next Fibonacci calculation if true (index 8 is hypothetical and needs adjustment)
            // Calculation part: add the last two Fibonacci numbers
            OpCode::Dup,  // Duplicate the last Fibonacci number
            OpCode::Swap, // Swap the two topmost numbers to get the second last
            OpCode::Dup,  // Duplicate the second last Fibonacci number
            OpCode::Swap, // Restore original order
            OpCode::Add,  // Add the two topmost numbers to get the next Fibonacci number
            // Counter increment and conditional jump back to loop start
            OpCode::Push(Value::Int32(1)), // Push 1 to increment the counter
            OpCode::Add,                   // Increment the counter
            OpCode::Jmp(4), // Jump back to the start of the loop (index 4 is hypothetical and needs adjustment)
            // End of loop; cleanup stack to leave the last calculated Fibonacci number on top
            OpCode::NoOp, // Placeholder for additional cleanup or result preparation opcodes
        ];

        // Execute the Fibonacci sequence program
        vm.execute(&program);

        // Assuming the last Fibonacci number is left on top of the stack
        assert_eq!(vm.pop(), Value::Int32(5)); // Check if the 5th Fibonacci number is indeed 5
    }
}
