use pest::iterators::Pairs;
use pest::iterators::Pair;
use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"] // Ensure this path is correct
pub struct MyLanguageParser;

pub fn parse(source: &str) -> Program {
    let pairs = MyLanguageParser::parse(Rule::program, source).unwrap_or_else(|e| panic!("{}", e));
    parse_program_pairs(pairs)
}

pub fn parse_program_pairs(pairs: Pairs<Rule>) -> Program {
    let declarations = pairs.map(parse_declaration).collect();
    Program::Declarations(declarations)
}

pub fn parse_program(pair: Pair<Rule>) -> Program {
    let declarations = pair.into_inner().map(parse_declaration).collect();
    Program::Declarations(declarations)
}

fn parse_declaration(pair: Pair<Rule>) -> Declaration {
    match pair.as_rule() {
        Rule::trait_declaration => parse_trait_declaration(pair),
        Rule::class_declaration => parse_class_declaration(pair),
        _ => unreachable!(),
    }
}

fn parse_trait_declaration(pair: Pair<Rule>) -> Declaration {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let methods = inner.map(parse_method_signature).collect();
    
    Declaration::TraitDecl(TraitDecl {
        name,
        methods,
    })
}

fn parse_class_declaration(pair: Pair<Rule>) -> Declaration {
    let mut inner = pair.into_inner();
    let access_modifier = inner.next().unwrap(); // Assuming all classes have access modifiers for simplicity
    let name = inner.next().unwrap().as_str().to_string();
    let superclass = inner.peek().map(|p| p.as_str().to_string()); // Optional inheritance
    let members = inner.map(parse_class_member).collect();
    
    Declaration::ClassDecl(ClassDecl {
        access_modifier: parse_access_modifier(access_modifier),
        name,
        superclass,
        members,
    })
}

fn parse_method_signature(pair: Pair<Rule>) -> MethodSignature {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let params = parse_parameter_list(inner.next().unwrap());
    let return_type = parse_type(inner.next().unwrap());
    
    MethodSignature {
        name,
        params,
        return_type,
    }
}

fn parse_constructor_declaration(pair: Pair<Rule>) -> ClassMember {
    let mut inner = pair.into_inner();
    let access_modifier = inner.next().map(parse_access_modifier).unwrap();
    let params = parse_parameter_list(inner.next().unwrap());
    let body = parse_block(inner.next().unwrap());
    
    ClassMember::Constructor(ConstructorDecl {
        access_modifier,
        params,
        body,
    })
}

fn parse_property_declaration(pair: Pair<Rule>) -> ClassMember {
    let mut inner = pair.into_inner();
    let access_modifier = inner.next().map(parse_access_modifier).unwrap();
    let is_static = inner.next().is_some();
    let name = inner.next().unwrap().as_str().to_string();
    let type_ = parse_type(inner.next().unwrap());
    let initializer = parse_expression(inner.next().unwrap());
    
    ClassMember::Property(PropertyDecl {
        access_modifier,
        is_static,
        name,
        type_,
        initializer,
    })
}

fn parse_abstract_method_declaration(pair: Pair<Rule>) -> ClassMember {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let params = parse_parameter_list(inner.next().unwrap());
    let return_type = parse_type(inner.next().unwrap());
    
    ClassMember::AbstractMethod(MethodSignature {
        name,
        params,
        return_type,
    })
}

fn parse_class_member(pair: Pair<Rule>) -> ClassMember {
    match pair.as_rule() {
        Rule::constructor_declaration => parse_constructor_declaration(pair),
        Rule::method_declaration => ClassMember::Method(parse_method_declaration(pair)),
        Rule::property_declaration => parse_property_declaration(pair),
        Rule::abstract_method_declaration => parse_abstract_method_declaration(pair),
        _ => unreachable!(),
    }
}

fn parse_access_modifier(pair: Pair<Rule>) -> Option<AccessModifier> {
    match pair.as_str() {
        "public" => Some(AccessModifier::Public),
        "private" => Some(AccessModifier::Private),
        "protected" => Some(AccessModifier::Protected),
        _ => None,
    }
}


// Sample parsing function update for method declarations with access modifiers and static keyword
fn parse_method_declaration(pair: Pair<Rule>) -> MethodDecl {
    let mut access_modifier = None;
    let mut is_static = false;
    let mut inner = pair.into_inner();

    // Check for access modifier
    if let Some(am_pair) = inner.peek() {
        if am_pair.as_rule() == Rule::access_modifier {
            access_modifier = Some(parse_access_modifier(inner.next().unwrap())).unwrap();
        }
    }

    // Check for static modifier
    if let Some(static_pair) = inner.peek() {
        if static_pair.as_str() == "static" {
            is_static = true;
            inner.next(); // Consume the static keyword
        }
    }

    // Continue with parsing method name, parameters, return type, and body...
    let name = inner.next().unwrap().as_str().to_string();
    let params = parse_parameter_list(inner.next().unwrap());
    let return_type = parse_type(inner.next().unwrap());
    let body = parse_block(inner.next().unwrap());

    MethodDecl {
        access_modifier,
        is_static,
        name,
        params,
        return_type,
        body,
    }

}

fn parse_parameter_list(pair: Pair<Rule>) -> Vec<(String, Type)> {
    pair.into_inner().map(|p| {
        let mut inner = p.into_inner();
        let name = inner.next().unwrap().as_str().to_string();
        let type_ = parse_type(inner.next().unwrap());
        (name, type_)
    }).collect()
}

fn parse_block(pair: Pair<Rule>) -> Vec<Stmt> {
    pair.into_inner().map(parse_statement).collect()
}

fn parse_statement(pair: Pair<Rule>) -> Stmt {
    match pair.as_rule() {
        Rule::variable_declaration => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let type_pair = inner.next().unwrap();
            let var_type = match type_pair.as_rule() {
                Rule::r#type => parse_type(type_pair),
                _ => unreachable!(),
            };
            let expr = parse_expression(inner.next().unwrap());
            Stmt::VarDecl(name, var_type, expr)
        },
        Rule::expression => Stmt::Expr(parse_expression(pair)),
        _ => unreachable!(),
    }
}

fn parse_type(pair: Pair<Rule>) -> Type {
    match pair.as_str() {
        "uint8" => Type::Uint8,
        "uint16" => Type::Uint16,
        "uint32" => Type::Uint32,
        "uint64" => Type::Uint64,
        "uint128" => Type::Uint128,
        "bool" => Type::Bool,
        _ => unreachable!(),
    }
}

fn parse_expression(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::integer_literal => Expr::IntegerLiteral(pair.as_str().parse::<i64>().unwrap()),
        Rule::expression | Rule::term | Rule::factor => {
            let mut inner = pair.into_inner();
            let mut expr = parse_expression(inner.next().unwrap());
            while let Some(op_pair) = inner.next() {
                let op = match op_pair.as_rule() {
                    Rule::add => Operator::Add,
                    Rule::subtract => Operator::Subtract,
                    Rule::multiply => Operator::Multiply,
                    Rule::divide => Operator::Divide,
                    _ => unreachable!(),
                };
                let right = parse_expression(inner.next().unwrap());
                expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
            }
            expr
        },
        Rule::identifier => Expr::Variable(pair.as_str().to_string()),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    IntegerLiteral(i64),
    Variable(String),
    BinaryOp(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
enum Stmt {
    VarDecl(String, Type, Expr),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Program {
    Declarations(Vec<Declaration>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    ClassDecl(ClassDecl),
    TraitDecl(TraitDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    access_modifier: Option<AccessModifier>,
    name: String,
    superclass: Option<String>, // For inheritance
    members: Vec<ClassMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDecl {
    name: String,
    methods: Vec<MethodSignature>,
}

#[derive(Debug, Clone, PartialEq)]
struct MethodSignature {
    name: String,
    params: Vec<(String, Type)>,
    return_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
enum ClassMember {
    Constructor(ConstructorDecl),
    Method(MethodDecl),
    Property(PropertyDecl),
    AbstractMethod(MethodSignature), // Abstract methods don't have a body
}

#[derive(Debug, Clone, PartialEq)]
struct ConstructorDecl {
    access_modifier: Option<AccessModifier>,
    params: Vec<(String, Type)>,
    body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
struct MethodDecl {
    access_modifier: Option<AccessModifier>,
    is_static: bool,
    name: String,
    params: Vec<(String, Type)>,
    return_type: Type,
    body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
struct PropertyDecl {
    access_modifier: Option<AccessModifier>,
    is_static: bool,
    name: String,
    type_: Type,
    initializer: Expr,
}

#[derive(Debug, Clone, PartialEq)]
enum AccessModifier {
    Public,
    Private,
    Protected,
}
