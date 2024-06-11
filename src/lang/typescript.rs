// Assuming the OpCode and Value definitions from your stack machine are in scope

use crate::vm::OpCode;
use crate::vm::Value;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::vec;

#[derive(Parser)]
#[grammar = "fhescript.pest"]
pub struct StackScriptParser;

// Assuming existing definitions for ASTNode and CompilerContext

// Parse and build AST from StackScript code

fn parse_stackscript(code: &str) -> Vec<ASTNode> {
    let pairs = StackScriptParser::parse(Rule::program, code).unwrap_or_else(|e| panic!("{}", e));
    pairs.map(parse_value).collect()
}

fn parse_value(pair: Pair<Rule>) -> ASTNode {
    println!("Rule:    {:?}", pair.as_rule());
    println!("Span:    {:?}", pair.as_span());
    println!("Text:    {}\n", pair.as_str());
    match pair.as_rule() {
        Rule::statement => {
            //let mut inner_rules = pair.into_inner();
            parse_value(pair)
        }
        Rule::boolean_expression => parse_boolean_expression(pair),
        Rule::variable_assignment => parse_variable_assignment(pair),
        Rule::if_statement => parse_if_statement(pair),
        Rule::term => parse_term(pair),
        Rule::factor => parse_factor(pair),
        Rule::arithmetic_expression => parse_arithmetic_expression(pair),
        Rule::identifier => ASTNode::Variable(pair.as_str().to_owned()),
        Rule::number => ASTNode::Number(pair.as_str().parse().unwrap()),
        Rule::WHITESPACE => ASTNode::Ignore,
        Rule::COMMENT => ASTNode::Ignore,
        Rule::EOI => ASTNode::Ignore,
        _ => unimplemented!(),
    }
}

// Simplified representation of an AST node for StackScript
#[derive(Debug)]
enum ASTNode {
    Number(i32),
    Statement {
        expr: Box<ASTNode>,
    },
    Factor {
        id: Box<ASTNode>,
        expr: Box<ASTNode>,
    },
    Term {
        op: char,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    BooleanExpression {
        op: String,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    ArithmeticExpression {
        op: char,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Variable(String),
    Assignment {
        name: String,
        value: Box<ASTNode>,
    },
    If {
        condition: Box<ASTNode>,
        then_branch: Vec<ASTNode>,
    },
    Ignore,
}

fn parse_identifier(pair: Pair<Rule>) -> ASTNode {
    ASTNode::Variable(pair.as_str().to_owned())
}

fn parse_number(pair: Pair<Rule>) -> ASTNode {
    ASTNode::Number(pair.as_str().parse().unwrap())
}

fn parse_variable(pair: Pair<Rule>) -> ASTNode {
    ASTNode::Variable(pair.as_str().to_owned())
}

fn parse_variable_assignment(pair: Pair<Rule>) -> ASTNode {
    let mut inner_rules = pair.into_inner();
    let name = inner_rules.next().unwrap().as_str().to_owned();
    let value = Box::new(parse_variable(inner_rules.next().unwrap()));
    ASTNode::Assignment { name, value }
}

fn parse_factor(pair: Pair<Rule>) -> ASTNode {
    let mut inner_rules = pair.into_inner();
    let id = Box::new(parse_identifier(inner_rules.next().unwrap()));
    let expr = Box::new(parse_arithmetic_expression(inner_rules.next().unwrap()));
    ASTNode::Factor { id, expr }
}

fn parse_term(pair: Pair<Rule>) -> ASTNode {
    let mut inner_rules = pair.into_inner();
    let left = Box::new(parse_factor(inner_rules.next().unwrap()));
    let op = inner_rules.next().unwrap().as_str().chars().next().unwrap();
    let right = Box::new(parse_factor(inner_rules.next().unwrap()));
    ASTNode::Term { op, left, right }
}

fn parse_arithmetic_expression(pair: Pair<Rule>) -> ASTNode {
    let mut inner_rules = pair.into_inner();
    let left = Box::new(parse_term(inner_rules.next().unwrap()));
    let op = inner_rules.next().unwrap().as_str().chars().next().unwrap();
    let right = Box::new(parse_term(inner_rules.next().unwrap()));
    ASTNode::ArithmeticExpression { op, left, right }
}

fn parse_boolean_expression(pair: Pair<Rule>) -> ASTNode {
    let mut inner_rules = pair.into_inner();
    let left = Box::new(parse_arithmetic_expression(inner_rules.next().unwrap()));
    let op = inner_rules.next().unwrap().as_str().to_owned();
    let right = Box::new(parse_arithmetic_expression(inner_rules.next().unwrap()));
    ASTNode::BooleanExpression { op, left, right }
}

fn parse_if_statement(pair: Pair<Rule>) -> ASTNode {
    let mut inner_rules = pair.into_inner();
    let condition = Box::new(parse_boolean_expression(inner_rules.next().unwrap()));

    let mut then_branch = vec![];
    for inner_rule in inner_rules {
        println!("Inner Rule: {:?}", inner_rule.as_rule());
        then_branch.push(parse_statement(inner_rule.into_inner().next().unwrap()));
    }

    //let then_branch = Box::new(parse_statement(inner_rules.next().unwrap()));
    ASTNode::If {
        condition,
        then_branch,
    }
}

fn parse_statement(pair: Pair<Rule>) -> ASTNode {
    let mut inner_rules = pair.into_inner();
    let expr = Box::new(parse_arithmetic_expression(inner_rules.next().unwrap()));
    ASTNode::Statement { expr }
}

// Context for variable addresses
struct CompilerContext {
    variables: HashMap<String, i32>,
    next_var_address: i32,
}

impl CompilerContext {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
            next_var_address: 0,
        }
    }

    // Allocate a new variable or get the existing address
    fn alloc_var(&mut self, name: &str) -> i32 {
        match self.variables.get(name) {
            Some(&address) => address,
            None => {
                let address = self.next_var_address;
                self.variables.insert(name.to_owned(), address);
                self.next_var_address += 1;
                address
            }
        }
    }
}

// Example function to compile an ASTNode to OpCode sequence
fn compile_node(node: &ASTNode, context: &mut CompilerContext) -> Vec<OpCode> {
    match node {
        ASTNode::Statement { expr } => compile_node(expr, context),
        ASTNode::Factor { id, expr } => {
            let mut code = Vec::new();
            code.extend(compile_node(id, context));
            code.extend(compile_node(expr, context));
            code
        }
        ASTNode::Term { op, left, right } => {
            let mut code = Vec::new();
            code.extend(compile_node(left, context));
            code.extend(compile_node(right, context));
            match op {
                '+' => code.push(OpCode::Add),
                '-' => code.push(OpCode::Sub),
                '*' => code.push(OpCode::Mul),
                '/' => code.push(OpCode::Div),
                // Add other operations as needed
                _ => unimplemented!(),
            }
            code
        }
        ASTNode::Ignore => Vec::new(),
        ASTNode::Number(n) => vec![OpCode::Push(Value::Int32(*n))],
        ASTNode::ArithmeticExpression { op, left, right } => {
            let mut code = Vec::new();
            code.extend(compile_node(left, context));
            code.extend(compile_node(right, context));
            match op {
                '+' => code.push(OpCode::Add),
                '-' => code.push(OpCode::Sub),
                '*' => code.push(OpCode::Mul),
                '/' => code.push(OpCode::Div),
                // Add other operations as needed
                _ => unimplemented!(),
            }
            code
        }
        ASTNode::BooleanExpression { op, left, right } => {
            let mut code = Vec::new();
            code.extend(compile_node(left, context));
            code.extend(compile_node(right, context));
            match op.as_str() {
                ">" => code.push(OpCode::Gt),
                "<" => code.push(OpCode::Lt),
                "==" => code.push(OpCode::Eq),
                "!=" => code.push(OpCode::Neq),
                ">=" => code.push(OpCode::Gte),
                "<=" => code.push(OpCode::Lte),
                // Add other operations as needed
                _ => unimplemented!(),
            }
            code
        }

        ASTNode::Variable(name) => {
            let address = context.alloc_var(name);
            vec![OpCode::Load(address)]
        }
        ASTNode::Assignment { name, value } => {
            let address = context.alloc_var(name);
            let mut code = compile_node(value, context);
            code.push(OpCode::Store(address));
            code
        }
        ASTNode::If {
            condition,
            then_branch,
        } => {
            let mut code = compile_node(condition, context);
            // Placeholder for jump logic; needs additional logic to calculate jump addresses
            code.push(OpCode::JmpIf(0)); // Placeholder
            for stmt in then_branch {
                code.extend(compile_node(stmt, context));
            }
            code
        } // Add more node types as needed
    }
}

// Function to compile a whole StackScript program (a series of ASTNodes)
fn compile_program(program: &[ASTNode]) -> Vec<OpCode> {
    let mut context = CompilerContext::new();
    program
        .iter()
        .flat_map(|node| compile_node(node, &mut context))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::VM;

    #[test]
    fn test_parser_with_ast2() {
        let code = "
            let a = 35;
            let a = a + 5;
            if (a > 25) {
                let a = a - 20;
            }
        ";
        let ast = parse_stackscript(code);
        // Print the AST for debugging
        println!("{:?}", ast);

        let compiled_opcodes = compile_program(&ast);

        compiled_opcodes.iter().for_each(|op| println!("{:?}", op));
    }

    #[test]
    fn test_parser_with_ast() {
        let code = "
            let a = 10;
            let b = 20;
            let a = a + b;
            if (a > 25) {
                let a = a - 5;
            }
        ";
        let ast = parse_stackscript(code);
        // Print the AST for debugging
        println!("{:?}", ast);

        let compiled_opcodes = compile_program(&ast);

        compiled_opcodes.iter().for_each(|op| println!("{:?}", op));

        let mut vm = VM::new();

        vm.execute(&compiled_opcodes);

        if let Some(result) = vm.stack.last() {
            println!("Result of addition: {:?}", result);
        } else {
            println!("Error: Stack is empty.");
        }
    }
}
