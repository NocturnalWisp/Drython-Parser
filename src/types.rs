use std::collections::HashMap;

use super::script_type::ScriptType;

pub mod error;

#[derive(Debug)]
pub struct Parser
{
    pub script_type: ScriptType,
    pub global_expressions: ExpressionList
}

#[derive(Debug)]
pub struct ExpressionList
{
    pub scope_info: (Option<String>, Option<String>),
    pub size: usize,
    pub line_start: usize,
    // Return, assignment
    pub single_op: HashMap<usize, (String, Vec<Token>, usize)>,
    // Function call
    pub multi_ops: HashMap<usize, (String, Vec<Vec<Token>>, usize)>,
    pub internal_expressions: HashMap<usize, (ExpressionList, usize)>,
    pub includes: HashMap<String, usize>,
}

impl ExpressionList
{
    pub fn new() -> ExpressionList
    {
        ExpressionList
        {
            scope_info: (None, None),
            size: 0,
            line_start: 0,
            single_op: HashMap::new(),
            multi_ops: HashMap::new(),
            internal_expressions: HashMap::new(),
            includes: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token
{
    Null,
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Char(char),
    Collection(Vec<Token>),
    
    // Meta variables that store information not usually visible to the dev.
    // Unless debugging.
    Var(String),
    Call(String, String),
    Operation(Vec<Token>),
    Operator(String),
    // Accessor stores the accessor after the '.', and the token before.
    Accessor(Box<Token>, Box<Token>),

    // Internal use only.
    // Allows an inner scope to break a loop.
    Break
}

pub type DynamicFunctionCall = Option<Box<dyn Fn(Vec<Token>) -> Result<Option<Token>, String>>>;
pub type RegisteredFunction= (String, DynamicFunctionCall);
pub type RegisteredVariable = (String, Token);


#[derive(Debug)]
pub struct Operation
{
    pub a: Token,
    pub b: Token,
}

pub struct Runner
{
    pub parser: Parser,
    pub external_functions: HashMap<String, DynamicFunctionCall>,
    pub vars: HashMap<String, Token>
}
