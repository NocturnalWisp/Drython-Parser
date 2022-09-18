use std::collections::HashMap;

use super::script_type::ScriptType;

pub mod error;

#[derive(Debug)]
pub struct Parser
{
    pub script_type: ScriptType,
    pub global_expressions: ExpressionList
}

#[derive(Clone, Debug)]
pub enum ExpressionListType
{
    Null, // For empty lines to properly track line number.
    Single,
    Multi,
    Internal,
    Library,
}

pub type ScopeInfo = (Option<String>, Option<String>);
pub type SingleOp = (String, Vec<String>, Vec<Token>, usize);
pub type MultiOp = (String, Vec<Vec<Token>>, usize);
pub type Internal = (ExpressionList, usize);

#[derive(Clone, Debug)]
pub struct ExpressionList
{
    pub scope_info: ScopeInfo,
    pub size: usize,
    pub line_start: usize,

    pub expression_order: Vec<ExpressionListType>,
    // Return, assignment
    pub single_op: Vec<SingleOp>,
    // Function call
    pub multi_ops: Vec<MultiOp>,
    pub internal_expressions: Vec<Internal>,
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

            expression_order: Vec::new(),
            single_op: Vec::new(),
            multi_ops: Vec::new(),
            internal_expressions: Vec::new(),
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

pub trait ExFnRef
{
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableModifier
{
    Unkown(String),
    // Able to externally reference other scripts.
    External,
    // Listable with other public values for external use.
    Public,
    // Make the variable non modifiable.
    Const,
    // Define aliases for variables and functions.
    Alias,
}

impl From<&str> for VariableModifier
{
    fn from(s: &str) -> Self
    {
        match s.to_lowercase().as_str()
        {
            "external" => VariableModifier::External,
            "pub"|"public" => VariableModifier::Public,
            "const"|"constant" => VariableModifier::Const,
            "alias" => VariableModifier::Alias,
            _ => VariableModifier::Unkown(s.to_string())
        }
    }
}

impl VariableModifier
{
    pub fn check_scope_allowed(&self) -> bool
    {
        match self
        {
            VariableModifier::Const => true,
            VariableModifier::Alias => true,
            VariableModifier::Public => false,
            VariableModifier::External => false,
            _ => false,
        }
    }
}

pub type VarMap = HashMap<String, (Token, bool, Vec<VariableModifier>)>;

pub type BoxedCall = Box<dyn Fn(Option<*mut dyn ExFnRef>, Vec<Token>) -> Result<Option<Token>, String>>;
pub type DynamicFunctionCall = (Option<*mut dyn ExFnRef>,
                                Option<BoxedCall>);
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
    // bool - is external var
    // external vars cannot have their types changed.
    pub vars: VarMap,
    pub var_indexes_changed: Vec<String>,
}
