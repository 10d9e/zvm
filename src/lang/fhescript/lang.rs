use crate::vm::OpCode;
use crate::vm::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    IntKeyword,
    FunctionKeyword,
    IfKeyword,
    ElseKeyword,
    ReturnKeyword,
    Identifier(String),
    Integer(i32),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Colon,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpGt,
    OpLt,
    Assign,
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut number = String::new();
                while let Some('0'..='9') = chars.peek() {
                    number.push(chars.next().unwrap());
                }
                tokens.push(Token::Integer(number.parse().unwrap()));
            },
            'a'..='z' | 'A'..='Z' => {
                let mut identifier = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        identifier.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                match identifier.as_str() {
                    "int" => tokens.push(Token::IntKeyword),
                    "function" => tokens.push(Token::FunctionKeyword),
                    "if" => tokens.push(Token::IfKeyword),
                    "else" => tokens.push(Token::ElseKeyword),
                    "return" => tokens.push(Token::ReturnKeyword),
                    _ => tokens.push(Token::Identifier(identifier)),
                }
            },
            '+' => { tokens.push(Token::OpAdd); chars.next(); },
            '-' => { tokens.push(Token::OpSub); chars.next(); },
            '*' => { tokens.push(Token::OpMul); chars.next(); },
            '/' => { tokens.push(Token::OpDiv); chars.next(); },
            '>' => { tokens.push(Token::OpGt); chars.next(); },
            '<' => { tokens.push(Token::OpLt); chars.next(); },
            '=' => { tokens.push(Token::Assign); chars.next(); },
            '(' => { tokens.push(Token::LParen); chars.next(); },
            ')' => { tokens.push(Token::RParen); chars.next(); },
            '{' => { tokens.push(Token::LBrace); chars.next(); },
            '}' => { tokens.push(Token::RBrace); chars.next(); },
            ',' => { tokens.push(Token::Comma); chars.next(); },
            ';' => { tokens.push(Token::Semicolon); chars.next(); },
            ':' => { tokens.push(Token::Colon); chars.next(); },
            ' ' | '\n' | '\t' => { chars.next(); }, // Skip whitespace
            _ => { /* handle errors */ }
        }
    }
    tokens
}

#[derive(Debug, Clone, PartialEq)]
enum AstNode {
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<AstNode>,
        return_type: ValueType,
    },
    VariableDeclaration {
        name: String,
        vtype: ValueType,
        value: Box<AstNode>,
    },
    BinaryExpr {
        left: Box<AstNode>,
        operator: BinaryOp,
        right: Box<AstNode>,
    },
    LiteralInt(i32),
    VariableRef(String),
    Return(Box<AstNode>),
    If {
        condition: Box<AstNode>,
        then_branch: Vec<AstNode>,
        else_branch: Option<Vec<AstNode>>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ValueType {
    Int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

fn parse(tokens: &[Token]) -> Vec<AstNode> {
    // This is a placeholder. A real implementation should correctly parse tokens into AST nodes.
    vec![] // Return an empty AST for simplicity here.
}

fn compile(ast: &[AstNode]) -> Vec<OpCode> {
    let mut opcodes = Vec::new();
    for node in ast {
        match node {
            AstNode::Function { name, params, body, return_type } => {
                // Handle function declaration
                // Functions could be implemented as jumps in VM code
                opcodes.push(OpCode::NoOp); // Placeholder
            },
            AstNode::VariableDeclaration { name, vtype, value } => {
                // Assume all variables are stored in a fixed position in memory
                // We compile the expression to push its result on the stack
                opcodes.extend(compile(&[*value.clone()]));
                // We then store the result in a predefined address
                // (This would need a more complex system to manage memory addresses)
                opcodes.push(OpCode::Store(0)); // Placeholder address
            },
            AstNode::BinaryExpr { left, operator, right } => {
                // Compile left and right expressions
                opcodes.extend(compile(&[*left.clone()]));
                opcodes.extend(compile(&[*right.clone()]));
                // Apply the operator
                match operator {
                    BinaryOp::Add => opcodes.push(OpCode::Add),
                    BinaryOp::Sub => opcodes.push(OpCode::Sub),
                    BinaryOp::Mul => opcodes.push(OpCode::Mul),
                    BinaryOp::Div => opcodes.push(OpCode::Div),
                }
            },
            AstNode::LiteralInt(value) => {
                // Push a literal integer onto the stack
                opcodes.push(OpCode::Push(Value::Int32(*value)));
            },
            AstNode::VariableRef(name) => {
                // Assume the variable is at a fixed memory location
                opcodes.push(OpCode::Load(0)); // Placeholder address
            },
            AstNode::Return(expression) => {
                // Compile the return expression
                opcodes.extend(compile(&[*expression.clone()]));
                // In a real function, we would also handle function stack cleanup
            },
            AstNode::If { condition, then_branch, else_branch } => {
                // Compile the condition
                opcodes.extend(compile(&[*condition.clone()]));
                // Opcode to jump to else branch if condition is false
                let jump_to_else = OpCode::JmpIf(0); // Placeholder, needs real address calculation
                opcodes.push(jump_to_else);
                // Compile the then branch
                opcodes.extend(compile(then_branch));
                // Jump over else branch when then branch is done
                let jump_over_else = OpCode::Jmp(0); // Placeholder
                opcodes.push(jump_over_else);
                // Compile the else branch if it exists
                if let Some(else_nodes) = else_branch {
                    opcodes.extend(compile(else_nodes));
                }
            },
        }
    }
    opcodes
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_lex_basic_expression() {
        let input = "int x = 10 + 20;";
        let expected_tokens = vec![
            Token::IntKeyword,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Integer(10),
            Token::OpAdd,
            Token::Integer(20),
            Token::Semicolon,
        ];
        let result = lex(input);
        assert_eq!(result, expected_tokens);
    }

    #[test]
    fn test_lex_function_declaration() {
        let input = "function max(a: int, b: int): int { if (a > b) { return a; } else { return b; } }";
        let expected_tokens = vec![
            Token::FunctionKeyword,
            Token::Identifier("max".to_string()),
            Token::LParen,
            Token::Identifier("a".to_string()),
            Token::Colon,
            Token::IntKeyword,
            Token::Comma,
            Token::Identifier("b".to_string()),
            Token::Colon,
            Token::IntKeyword,
            Token::RParen,
            Token::Colon,
            Token::IntKeyword,
            Token::LBrace,
            Token::IfKeyword,
            Token::LParen,
            Token::Identifier("a".to_string()),
            Token::OpGt,
            Token::Identifier("b".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::ReturnKeyword,
            Token::Identifier("a".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::ElseKeyword,
            Token::LBrace,
            Token::ReturnKeyword,
            Token::Identifier("b".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::RBrace,
        ];
        let result = lex(input);
        assert_eq!(result, expected_tokens);
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_parse_variable_declaration() {
        let tokens = vec![
            Token::IntKeyword,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Integer(42),
            Token::Semicolon,
        ];
        let expected_ast = vec![
            AstNode::VariableDeclaration {
                name: "x".to_string(),
                vtype: ValueType::Int,
                value: Box::new(AstNode::LiteralInt(42)),
            },
        ];
        let result = parse(&tokens);
        assert_eq!(result, expected_ast);
    }
}

#[cfg(test)]
mod code_generation_tests {
    use super::*;

    #[test]
    fn test_codegen_simple_arithmetic() {
        let ast = vec![
            AstNode::BinaryExpr {
                left: Box::new(AstNode::LiteralInt(10)),
                operator: BinaryOp::Add,
                right: Box::new(AstNode::LiteralInt(20)),
            },
        ];
        let expected_opcodes = vec![
            OpCode::Push(Value::Int32(10)),
            OpCode::Push(Value::Int32(20)),
            OpCode::Add,
        ];
        let result = compile(&ast);
        assert_eq!(result, expected_opcodes);
    }
}
