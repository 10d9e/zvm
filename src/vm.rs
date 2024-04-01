use crate::value::Neg;
use crate::value::Value;
use serde::Deserialize;
use serde::Serialize;
use std::convert::TryInto;
use tfhe::prelude::*;
use tfhe::FheBool;

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
    /* TODO: I mean, can we please figure out a way to do oblivious Jmp and JmpIf?
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

impl std::fmt::Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Add => write!(f, "Add"),
            OpCode::Sub => write!(f, "Sub"),
            OpCode::Mul => write!(f, "Mul"),
            OpCode::Div => write!(f, "Div"),
            OpCode::Neg => write!(f, "Neg"),
            OpCode::And => write!(f, "And"),
            OpCode::Or => write!(f, "Or"),
            OpCode::Xor => write!(f, "Xor"),
            OpCode::ShiftRight => write!(f, "ShiftRight"),
            OpCode::ShiftLeft => write!(f, "ShiftLeft"),
            OpCode::Eq => write!(f, "Eq"),
            OpCode::Neq => write!(f, "Neq"),
            OpCode::Lt => write!(f, "Lt"),
            OpCode::Lte => write!(f, "Lte"),
            OpCode::Gt => write!(f, "Gt"),
            OpCode::Gte => write!(f, "Gte"),
            OpCode::Min => write!(f, "Min"),
            OpCode::Max => write!(f, "Max"),
            OpCode::Mux => write!(f, "Mux"),
            OpCode::Push(value) => match value {
                Value::Ebool(_) => {
                    write!(f, "Push(Ebool)")
                }
                Value::Euint8(_) => {
                    write!(f, "Push(Euint8)")
                }
                Value::Euint16(_) => {
                    write!(f, "Push(Euint16)")
                }
                Value::Euint32(_) => {
                    write!(f, "Push(Euint32)")
                }
                Value::Euint64(_) => {
                    write!(f, "Push(Euint64)")
                }
                Value::Euint128(_) => {
                    write!(f, "Push(Euint128)")
                }
                Value::Bool(val) => {
                    write!(f, "Push(Bool: {:?})", val)
                }
                Value::Uint8(val) => {
                    write!(f, "Push(Uint8: {:?})", val)
                }
                Value::Uint16(val) => {
                    write!(f, "Push(Uint16: {:?})", val)
                }
                Value::Uint32(val) => {
                    write!(f, "Push(Uint32: {:?})", val)
                }
                Value::Uint64(val) => {
                    write!(f, "Push(Uint64: {:?})", val)
                }
                Value::Uint128(val) => {
                    write!(f, "Push(Uint128: {:?})", val)
                }
            },
            OpCode::Dup => write!(f, "Dup"),
            OpCode::NoOp => write!(f, "NoOp"),
            OpCode::Inc => write!(f, "Inc"),
            OpCode::Dec => write!(f, "Dec"),
            OpCode::Load(address) => write!(f, "Load({})", address),
            OpCode::Store(address) => write!(f, "Store({})", address),
            OpCode::Swap => write!(f, "Swap"),
        }
    }
}

impl OpCode {
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
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
                    self.push(a.add_op(b));
                }
                OpCode::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.sub_op(b));
                }
                OpCode::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.mul_op(b));
                }
                OpCode::Div => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.div_op(b));
                }
                OpCode::And => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.and_op(b));
                }
                OpCode::Or => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.or_op(b));
                }
                OpCode::Xor => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.xor_op(b));
                }
                OpCode::Eq => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Ebool(a.eq(b));
                    self.push(val);
                }
                OpCode::Neq => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Ebool(a.ne(b));
                    self.push(val);
                }
                OpCode::Lt => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Ebool(a.lt(b));
                    self.push(val);
                }
                OpCode::Lte => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Ebool(a.le(b));
                    self.push(val);
                }
                OpCode::Gt => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Ebool(a.gt(b));
                    self.push(val);
                }
                OpCode::Gte => {
                    let b = self.pop();
                    let a = self.pop();
                    let val = Value::Ebool(a.ge(b));
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
                    self.push(a.shr_op(b));
                }
                OpCode::ShiftLeft => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.shl_op(b));
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
                        let fhe_one = FheBool::try_encrypt_trivial(false).unwrap();
                        self.memory.resize(uaddress + 1, Value::Ebool(fhe_one));
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
        let a = a.as_ebool().unwrap();
        let b = b.as_eint8().unwrap();
        let c = c.as_eint8().unwrap();
        Value::Euint8(a.if_then_else(b, c))
    }
}
