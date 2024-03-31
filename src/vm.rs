use serde::Deserialize;
use serde::Serialize;
use std::ops::Add;
use std::ops::Sub;
use tfhe::prelude::*;
use tfhe::{FheBool, FheUint128, FheUint16, FheUint32, FheUint64, FheUint8};

#[derive(Clone, Serialize, Deserialize)]
pub enum OpCode {
    // arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Neg,

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
    /* TODO?
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
            OpCode::Neg => vec![28],
        }
    }

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
            28 => (OpCode::Neg, 1),
            // Handle other opcodes...
            _ => unimplemented!(),
        }
    }
}

macro_rules! binary_op {
    ($($op:ident, $op_method:ident, $op_token:tt);*) => {
        impl Value {
            $(
                fn $op(self, other: Self) -> Self {
                    match (self, other) {

                        (Value::Bool(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Uint8(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Uint16(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Uint32(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Uint64(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Uint128(_b)) => unimplemented!(),

                        (Value::Uint8(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Uint8(a), Value::Uint8(b)) => Value::Uint8(a $op_token b),
                        (Value::Uint8(a), Value::Uint16(b)) => Value::Uint16(FheUint16::cast_from(a) $op_token b),
                        (Value::Uint8(a), Value::Uint32(b)) => Value::Uint32(FheUint32::cast_from(a) $op_token b),
                        (Value::Uint8(a), Value::Uint64(b)) => Value::Uint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Uint8(a), Value::Uint128(b)) => Value::Uint128(FheUint128::cast_from(a) $op_token b),

                        (Value::Uint16(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Uint16(a), Value::Uint8(b)) => Value::Uint16(a $op_token FheUint16::cast_from(b)),
                        (Value::Uint16(a), Value::Uint16(b)) => Value::Uint16(a $op_token b),
                        (Value::Uint16(a), Value::Uint32(b)) => Value::Uint32(FheUint32::cast_from(a) $op_token b),
                        (Value::Uint16(a), Value::Uint64(b)) => Value::Uint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Uint16(a), Value::Uint128(b)) => Value::Uint128(FheUint128::cast_from(a) $op_token b),

                        (Value::Uint32(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Uint32(a), Value::Uint8(b)) => Value::Uint32(a $op_token FheUint32::cast_from(b)),
                        (Value::Uint32(a), Value::Uint16(b)) => Value::Uint32(a $op_token FheUint32::cast_from(b)),
                        (Value::Uint32(a), Value::Uint32(b)) => Value::Uint32(a $op_token b),
                        (Value::Uint32(a), Value::Uint64(b)) => Value::Uint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Uint32(a), Value::Uint128(b)) => Value::Uint128(FheUint128::cast_from(a) $op_token b),

                        (Value::Uint64(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Uint64(a), Value::Uint8(b)) => Value::Uint64(a $op_token FheUint64::cast_from(b)),
                        (Value::Uint64(a), Value::Uint16(b)) => Value::Uint64(a $op_token FheUint64::cast_from(b)),
                        (Value::Uint64(a), Value::Uint32(b)) => Value::Uint64(a $op_token FheUint64::cast_from(b)),
                        (Value::Uint64(a), Value::Uint64(b)) => Value::Uint64(a $op_token b),
                        (Value::Uint64(a), Value::Uint128(b)) => Value::Uint128(FheUint128::cast_from(a) $op_token b),

                        (Value::Uint128(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Uint128(a), Value::Uint8(b)) => Value::Uint128(a $op_token FheUint128::cast_from(b)),
                        (Value::Uint128(a), Value::Uint16(b)) => Value::Uint128(a $op_token FheUint128::cast_from(b)),
                        (Value::Uint128(a), Value::Uint32(b)) => Value::Uint128(a $op_token FheUint128::cast_from(b)),
                        (Value::Uint128(a), Value::Uint64(b)) => Value::Uint128(a $op_token FheUint128::cast_from(b)),
                        (Value::Uint128(a), Value::Uint128(b)) => Value::Uint128(a $op_token b),
                    }
                }
            )*
        }
    };
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Value {
    Bool(FheBool),
    Uint8(FheUint8),
    Uint16(FheUint16),
    Uint32(FheUint32),
    Uint64(FheUint64),
    Uint128(FheUint128),
}

impl FheEq<Value> for Value {
    fn eq(&self, other: Self) -> FheBool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a.eq(b),
            (Value::Uint8(a), Value::Uint8(b)) => a.eq(b),
            (Value::Uint16(a), Value::Uint16(b)) => a.eq(b),
            (Value::Uint32(a), Value::Uint32(b)) => a.eq(b),
            (Value::Uint64(a), Value::Uint64(b)) => a.eq(b),
            (Value::Uint128(a), Value::Uint128(b)) => a.eq(b),
            _ => unimplemented!(),
        }
    }

    fn ne(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a.ne(b),
            (Value::Uint8(a), Value::Uint8(b)) => a.ne(b),
            (Value::Uint16(a), Value::Uint16(b)) => a.ne(b),
            (Value::Uint32(a), Value::Uint32(b)) => a.ne(b),
            (Value::Uint64(a), Value::Uint64(b)) => a.ne(b),
            (Value::Uint128(a), Value::Uint128(b)) => a.ne(b),
            _ => unimplemented!(),
        }
    }
}

impl FheMax<Value> for Value {
    type Output = Value;
    fn max(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Uint8(a), Value::Uint8(b)) => Value::Uint8(a.max(&b)),
            (Value::Uint16(a), Value::Uint16(b)) => Value::Uint16(a.max(&b)),
            (Value::Uint32(a), Value::Uint32(b)) => Value::Uint32(a.max(&b)),
            (Value::Uint64(a), Value::Uint64(b)) => Value::Uint64(a.max(&b)),
            (Value::Uint128(a), Value::Uint128(b)) => Value::Uint128(a.max(&b)),
            _ => unimplemented!(),
        }
    }
}

impl FheMin<Value> for Value {
    type Output = Value;
    fn min(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Uint8(a), Value::Uint8(b)) => Value::Uint8(a.min(&b)),
            (Value::Uint16(a), Value::Uint16(b)) => Value::Uint16(a.min(&b)),
            (Value::Uint32(a), Value::Uint32(b)) => Value::Uint32(a.min(&b)),
            (Value::Uint64(a), Value::Uint64(b)) => Value::Uint64(a.min(&b)),
            (Value::Uint128(a), Value::Uint128(b)) => Value::Uint128(a.min(&b)),
            _ => unimplemented!(),
        }
    }
}

impl FheOrd<Value> for Value {
    fn lt(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Uint8(a), Value::Uint8(b)) => a.lt(b),
            (Value::Uint16(a), Value::Uint16(b)) => a.lt(b),
            (Value::Uint32(a), Value::Uint32(b)) => a.lt(b),
            (Value::Uint64(a), Value::Uint64(b)) => a.lt(b),
            (Value::Uint128(a), Value::Uint128(b)) => a.lt(b),
            _ => unimplemented!(),
        }
    }

    fn le(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Uint8(a), Value::Uint8(b)) => a.le(b),
            (Value::Uint16(a), Value::Uint16(b)) => a.le(b),
            (Value::Uint32(a), Value::Uint32(b)) => a.le(b),
            (Value::Uint64(a), Value::Uint64(b)) => a.le(b),
            (Value::Uint128(a), Value::Uint128(b)) => a.le(b),
            _ => unimplemented!(),
        }
    }

    fn gt(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Uint8(a), Value::Uint8(b)) => a.gt(b),
            (Value::Uint16(a), Value::Uint16(b)) => a.gt(b),
            (Value::Uint32(a), Value::Uint32(b)) => a.gt(b),
            (Value::Uint64(a), Value::Uint64(b)) => a.gt(b),
            (Value::Uint128(a), Value::Uint128(b)) => a.gt(b),
            _ => unimplemented!(),
        }
    }

    fn ge(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Uint8(a), Value::Uint8(b)) => a.ge(b),
            (Value::Uint16(a), Value::Uint16(b)) => a.ge(b),
            (Value::Uint32(a), Value::Uint32(b)) => a.ge(b),
            (Value::Uint64(a), Value::Uint64(b)) => a.ge(b),
            (Value::Uint128(a), Value::Uint128(b)) => a.ge(b),
            _ => unimplemented!(),
        }
    }
}

impl Value {
    pub fn as_bool(&self) -> Option<FheBool> {
        match self {
            Value::Bool(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn as_int8(&self) -> Option<&FheUint8> {
        match self {
            Value::Uint8(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_int16(&self) -> Option<&FheUint16> {
        match self {
            Value::Uint16(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_int32(&self) -> Option<&FheUint32> {
        match self {
            Value::Uint32(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_int64(&self) -> Option<&FheUint64> {
        match self {
            Value::Uint64(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_int128(&self) -> Option<&FheUint128> {
        match self {
            Value::Uint128(value) => Some(value),
            _ => None,
        }
    }
}

trait Neg {
    fn neg(&self) -> Self;
}

impl Neg for Value {
    fn neg(&self) -> Self {
        match self {
            Value::Bool(val) => Value::Bool(!val),
            Value::Uint8(val) => Value::Uint8(-val),
            Value::Uint16(val) => Value::Uint16(-val),
            Value::Uint32(val) => Value::Uint32(-val),
            Value::Uint64(val) => Value::Uint64(-val),
            Value::Uint128(val) => Value::Uint128(-val),
        }
    }
}

impl Value {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Value::Bool(val) => {
                let mut bytes = vec![0];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Uint8(val) => {
                let mut bytes = vec![1];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Uint16(val) => {
                let mut bytes = vec![2];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Uint32(val) => {
                let mut bytes = vec![3];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Uint64(val) => {
                let mut bytes = vec![4];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Uint128(val) => {
                let mut bytes = vec![5];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
        }
    }

    fn from_bytes(bytes: &[u8]) -> (Self, usize) {
        // Returns Value and bytes consumed
        match bytes[0] {
            0 => {
                let val: FheBool = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Bool(val), 1)
            }
            1 => {
                let val: FheUint8 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Uint8(val), bytes.len())
            }
            2 => {
                let val: FheUint16 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Uint16(val), bytes.len())
            }
            3 => {
                let val: FheUint32 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Uint32(val), bytes.len())
            }
            4 => {
                let val: FheUint64 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Uint64(val), bytes.len())
            }
            5 => {
                let val: FheUint128 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Uint128(val), bytes.len())
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
            Value::Bool(_val) => unimplemented!(),
            Value::Uint8(val) => Value::Uint8(val + other),
            Value::Uint16(val) => Value::Uint16(val + other as u16),
            Value::Uint32(val) => Value::Uint32(val + other as u32),
            Value::Uint64(val) => Value::Uint64(val + other as u64),
            Value::Uint128(val) => Value::Uint128(val + other as u128),
        }
    }
}

impl Sub<u8> for Value {
    type Output = Self;

    fn sub(self, other: u8) -> Self {
        match self {
            Value::Bool(_val) => unimplemented!(),
            Value::Uint8(val) => Value::Uint8(val - other),
            Value::Uint16(val) => Value::Uint16(val - other as u16),
            Value::Uint32(val) => Value::Uint32(val - other as u32),
            Value::Uint64(val) => Value::Uint64(val - other as u64),
            Value::Uint128(val) => Value::Uint128(val - other as u128),
        }
    }
}

binary_op! {
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

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
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
            match &code[self.ip] {
                OpCode::Push(value) => {
                    self.stack.push(value.clone());
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
                    let val = Value::Bool(a.eq(b));
                    self.push(val);
                }
                OpCode::Neq => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Bool(a.ne(b));
                    self.push(val);
                }
                OpCode::Lt => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Bool(a.lt(b));
                    self.push(val);
                }
                OpCode::Lte => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Bool(a.le(b));
                    self.push(val);
                }
                OpCode::Gt => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Bool(a.gt(b));
                    self.push(val);
                }
                OpCode::Gte => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Bool(a.ge(b));
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
                    let result = self.mux(a, b, c);
                    self.push(result);
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
                    let value = self.stack.last().expect("Stack underflow on Dup");
                    self.stack.push(value.clone());
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
                    self.push(a - 1u8);
                }
                OpCode::Load(address) => {
                    // Assume memory is a Vec<Value>, and address is within bounds
                    let uaddress: usize = (*address).try_into().unwrap();
                    let value = &self.memory[uaddress];
                    self.stack.push(value.clone());
                }
                OpCode::Store(address) => {
                    let value = self.pop();
                    // Ensure memory is large enough to handle address
                    let uaddress: usize = (*address).try_into().unwrap();
                    if self.memory.len() <= uaddress {
                        let fhe_one: FheUint8 = FheUint8::try_encrypt_trivial(1u8).unwrap();
                        self.memory.resize(uaddress + 1, Value::Uint8(fhe_one));
                    }
                    self.memory[uaddress] = value;
                }
                OpCode::Swap => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a);
                    self.push(b);
                }
                OpCode::Neg => {
                    let a = self.pop();
                    self.push(a.neg());
                }
            }
            self.ip += 1; // Move to the next instruction unless jumped
        }
    }

    fn mux(&self, a: Value, b: Value, c: Value) -> Value {
        let a = a.as_bool().unwrap();
        let b = b.as_int8().unwrap();
        let c = c.as_int8().unwrap();

        let r = a.if_then_else(b, c);
        Value::Uint8(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheBool, FheUint8};

    #[test]
    fn test_add_i8_i16_promotion() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 20u8;
        let b = 5u16;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint16::try_encrypt(b, &client_key)?;

        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint16(enc_b)),
            OpCode::Add,
        ];
        vm.execute(&bytecode);

        let encrypted_res = vm.pop();
        let clear_res: u16 = encrypted_res.as_int16().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 25);

        Ok(())
    }

    #[test]
    fn test_add() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 1u8;
        let b = 2u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Add,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 3);
        Ok(())
    }

    #[test]
    fn test_subtract() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 20u8;
        let b = 5u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;

        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Sub,
        ];
        vm.execute(&bytecode);

        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 15);

        Ok(())
    }

    #[test]
    fn test_multiply() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 3u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Mul,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 6);
        Ok(())
    }

    #[test]
    fn test_divide() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 6u8;
        let b = 2u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Div,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 3);
        Ok(())
    }

    #[test]
    fn test_bitwise_and() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;
        let b = 3u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::And,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 1);
        Ok(())
    }

    #[test]
    fn test_bitwise_or() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;
        let b = 3u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Or,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 7);
        Ok(())
    }

    #[test]
    fn test_bitwise_xor() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;
        let b = 3u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Xor,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 6);
        Ok(())
    }

    #[test]
    fn test_bitwise_shift_left() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 1u8;
        let b = 1u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::ShiftLeft,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 2);
        Ok(())
    }

    #[test]
    fn test_bitwise_shift_right() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 1u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::ShiftRight,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 1);
        Ok(())
    }

    #[test]
    fn test_eq() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 2u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Eq,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res = encrypted_res.as_bool().unwrap().decrypt(&client_key);
        assert!(clear_res);
        Ok(())
    }

    #[test]
    fn test_neq() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 3u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Neq,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res = encrypted_res.as_bool().unwrap().decrypt(&client_key);
        assert!(clear_res);
        Ok(())
    }

    #[test]
    fn test_lt() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 1u8;
        let b = 2u8;
        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Lt,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res = encrypted_res.as_bool().unwrap().decrypt(&client_key);
        assert!(clear_res);
        Ok(())
    }

    #[test]
    fn test_gt() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 1u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Gt,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res = encrypted_res.as_bool().unwrap().decrypt(&client_key);
        assert!(clear_res);
        Ok(())
    }

    #[test]
    fn test_gte() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 2u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Gte,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res = encrypted_res.as_bool().unwrap().decrypt(&client_key);
        assert!(clear_res);
        Ok(())
    }

    #[test]
    fn test_lte() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 2u8;
        let b = 2u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Lte,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res = encrypted_res.as_bool().unwrap().decrypt(&client_key);
        assert!(clear_res);
        Ok(())
    }

    #[test]
    fn test_min() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 4u8;
        let b = 5u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Min,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 4);
        Ok(())
    }

    #[test]
    fn test_max() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 4u8;
        let b = 5u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Max,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 5);
        Ok(())
    }

    #[test]
    fn test_inc() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 4u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [OpCode::Push(Value::Uint8(enc_a)), OpCode::Inc];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 5);
        Ok(())
    }

    #[test]
    fn test_dec() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 4u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [OpCode::Push(Value::Uint8(enc_a)), OpCode::Dec];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 3);
        Ok(())
    }

    #[test]
    fn test_load() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Store(0),
            OpCode::Load(0),
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 5);
        Ok(())
    }

    #[test]
    fn test_store() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;
        let b = 6u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Store(0),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Store(1),
            OpCode::Load(0),
            OpCode::Load(1),
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 6);
        Ok(())
    }

    #[test]
    fn test_swap() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;
        let b = 6u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Uint8(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Swap,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 5);
        Ok(())
    }

    #[test]
    fn test_neg() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = 5u8;

        let enc_a = FheUint8::try_encrypt(a, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [OpCode::Push(Value::Uint8(enc_a)), OpCode::Neg];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, -5i8 as u8);
        Ok(())
    }

    #[test]
    fn test_mux() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        let a = true;
        let b = 6u8;
        let c = 1u8;

        let enc_a = FheBool::try_encrypt(a, &client_key)?;
        let enc_b = FheUint8::try_encrypt(b, &client_key)?;
        let enc_c = FheUint8::try_encrypt(c, &client_key)?;
        let mut vm = VM::new();
        let bytecode = [
            OpCode::Push(Value::Bool(enc_a)),
            OpCode::Push(Value::Uint8(enc_b)),
            OpCode::Push(Value::Uint8(enc_c)),
            OpCode::Mux,
        ];
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 6);
        Ok(())
    }

    #[test]
    fn test_program_serialization_deserialization() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
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
            //OpCode::Jmp(42),
            //OpCode::JmpIf(43),
            OpCode::Push(Value::Uint8(FheUint8::try_encrypt(8_u8, &client_key)?)),
            OpCode::Push(Value::Uint16(FheUint16::try_encrypt(16_u16, &client_key)?)),
            OpCode::Push(Value::Uint32(FheUint32::try_encrypt(32_u32, &client_key)?)),
            OpCode::Push(Value::Uint64(FheUint64::try_encrypt(64_u64, &client_key)?)),
            OpCode::Push(Value::Uint128(FheUint128::try_encrypt(
                128_u128,
                &client_key,
            )?)),
            OpCode::Dup,
            OpCode::NoOp,
            OpCode::Inc,
            OpCode::Dec,
            OpCode::Load(77),
            OpCode::Store(88),
            OpCode::Swap,
        ];

        // Serialize the program into bytes
        let serialized = serialize(&original_program);
        //println!("Serialized program: {:?}", hex::encode(serialized.clone()));

        // Deserialize the bytes back into opcodes
        let deserialized_program = deserialize(&serialized);

        // loop through deserialized program and assert each opcode matches original
        for (i, opcode) in deserialized_program.iter().enumerate() {
            assert_eq!(opcode.to_bytes(), original_program[i].to_bytes());
        }

        // Verify that the deserialized program matches the original
        //assert_eq!(deserialized_program, original_program);
        Ok(())
    }

    #[test]
    fn test_program_serialization_deserialization_types() -> Result<(), Box<dyn std::error::Error>>
    {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);
        // Define a sample program with a mix of opcodes, including a push with a value
        let original_program = [
            OpCode::Push(Value::Uint8(FheUint8::try_encrypt(8_u8, &client_key)?)),
            OpCode::Push(Value::Uint16(FheUint16::try_encrypt(16_u16, &client_key)?)),
            OpCode::Push(Value::Uint32(FheUint32::try_encrypt(32_u32, &client_key)?)),
            OpCode::Push(Value::Uint64(FheUint64::try_encrypt(64_u64, &client_key)?)),
            OpCode::Push(Value::Uint128(FheUint128::try_encrypt(
                128_u128,
                &client_key,
            )?)),
        ];

        // Serialize the program into bytes
        let serialized = serialize(&original_program);
        //println!("Serialized program: {:?}", hex::encode(serialized.clone()));

        // Deserialize the bytes back into opcodes
        let deserialized_program = deserialize(&serialized);

        // loop through deserialized program and assert each opcode matches original
        for (i, opcode) in deserialized_program.iter().enumerate() {
            assert_eq!(opcode.to_bytes(), original_program[i].to_bytes());
        }

        // Verify that the deserialized program matches the original
        //assert_eq!(deserialized_program, original_program);
        Ok(())
    }
    /*
    #[test]
    fn test_fibonacci() -> Result<(), Box<dyn std::error::Error>> {
        // Basic configuration to use homomorphic integers
        let config = ConfigBuilder::default().build();

        // Key generation
        let (client_key, server_keys) = generate_keys(config);
        // On the server side:
        set_server_key(server_keys);

        // Fibonacci program
        let bytecode = [
            OpCode::Push(Value::Uint8(FheUint8::try_encrypt(0, &client_key)?)), // n
            OpCode::Push(Value::Uint8(FheUint8::try_encrypt(1, &client_key)?)), // a
            OpCode::Push(Value::Uint8(FheUint8::try_encrypt(1, &client_key)?)), // b
            OpCode::Push(Value::Uint8(FheUint8::try_encrypt(10, &client_key)?)), // max
            OpCode::Store(0), // Store n
            OpCode::Store(1), // Store a
            OpCode::Store(2), // Store b
            OpCode::Store(3), // Store max
            OpCode::Load(0), // Load n
            OpCode::Load(3), // Load max
            OpCode::Gte, // n >= max
            OpCode::JmpIf(22), // Jump to end if n >= max
            OpCode::Load(1), // Load a
            OpCode::Load(2), // Load b
            OpCode::Add, // a + b
            OpCode::Store(4), // Store a + b
            OpCode::Load(2), // Load b
            OpCode::Store(1), // Store b
            OpCode::Load(4), // Load a + b
            OpCode::Store(2), // Store a + b
            OpCode::Load(0), // Load n
            OpCode::Inc, // n + 1
            OpCode::Store(0), // Store n + 1
            OpCode::Jmp(8), // Jump to beginning
            OpCode::Load(1), // Load a
        ];

        let mut vm = VM::new();
        vm.execute(&bytecode);
        let encrypted_res = vm.pop();
        let clear_res: u8 = encrypted_res.as_int8().unwrap().decrypt(&client_key);
        assert_eq!(clear_res, 55);
        Ok(())
    }
    */
}
