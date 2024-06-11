use std::str::FromStr;
use crate::vm::OpCode;
use crate::vm::Value;

// Compiler function to parse code into opcodes
pub fn compile(code: &str) -> Vec<OpCode> {
    let mut opcodes = Vec::new();
    for line in code.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            "push" => {
                if let Ok(num) = i32::from_str(parts[1]) {
                    opcodes.push(OpCode::Push(Value::Int32(num)));
                }
            },
            "add" => opcodes.push(OpCode::Add),
            "sub" => opcodes.push(OpCode::Sub),
            "mul" => opcodes.push(OpCode::Mul),
            "div" => opcodes.push(OpCode::Div),
            "and" => opcodes.push(OpCode::And),
            "or" => opcodes.push(OpCode::Or),
            "xor" => opcodes.push(OpCode::Xor),
            "shr" => opcodes.push(OpCode::ShiftRight),
            "shl" => opcodes.push(OpCode::ShiftLeft),
            "eq" => opcodes.push(OpCode::Eq),
            "neq" => opcodes.push(OpCode::Neq),
            "lt" => opcodes.push(OpCode::Lt),
            "lte" => opcodes.push(OpCode::Lte),
            "gt" => opcodes.push(OpCode::Gt),
            "gte" => opcodes.push(OpCode::Gte),
            "min" => opcodes.push(OpCode::Min),
            "max" => opcodes.push(OpCode::Max),
            "mux" => opcodes.push(OpCode::Mux),
            "jmp" => {
                if let Ok(idx) = i32::from_str(parts[1]) {
                    opcodes.push(OpCode::Jmp(idx));
                }
            },
            "jmpif" => {
                if let Ok(idx) = i32::from_str(parts[1]) {
                    opcodes.push(OpCode::JmpIf(idx));
                }
            },
            "dup" => opcodes.push(OpCode::Dup),
            "noop" => opcodes.push(OpCode::NoOp),
            "inc" => opcodes.push(OpCode::Inc),
            "dec" => opcodes.push(OpCode::Dec),
            "load" => {
                if let Ok(address) = i32::from_str(parts[1]) {
                    opcodes.push(OpCode::Load(address));
                }
            },
            "store" => {
                if let Ok(address) = i32::from_str(parts[1]) {
                    opcodes.push(OpCode::Store(address));
                }
            },
            "swap" => opcodes.push(OpCode::Swap),
            _ => eprintln!("Unknown instruction: {}", parts[0]),
        }
    }
    opcodes
}

fn main() {
    let program = "
        push 10
        push 20
        add
        store 0
        load 0
        push 1
        add
    ";

    let compiled = compile(program);
    for opcode in compiled.iter() {
        println!("{:?}", opcode);
    }
}

// write some tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile() {
        let program = "
            push 10
            push 20
            add
            store 0
            load 0
            push 1
            add
        ";

        let expected = vec![
            OpCode::Push(Value::Int32(10)),
            OpCode::Push(Value::Int32(20)),
            OpCode::Add,
            OpCode::Store(0),
            OpCode::Load(0),
            OpCode::Push(Value::Int32(1)),
            OpCode::Add,
        ];

        let compiled = compile(program);
        assert_eq!(compiled, expected);
    }
}
