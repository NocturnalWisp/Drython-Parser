#[path="types/expression_list.rs"]
mod expression_list;
pub use expression_list::ExpressionList as ExpressionList;
pub use expression_list::ExpressionListType as ExpressionListType;
pub use expression_list::ScopeInfo as ScopeInfo;
pub use expression_list::SingleOp as SingleOp;
pub use expression_list::MultiOp as MultiOp;
pub use expression_list::Internal as Internal;

#[path="types/variable_modifier.rs"]
mod variable_modifier;
pub use variable_modifier::VariableModifier as VariableModifier;

use std::collections::HashMap;

use super::script_type::ScriptType;

pub mod error;

#[derive(Debug)]
pub struct Parser
{
    pub script_type: ScriptType,
    pub global_expressions: ExpressionList
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

pub type VarMap = HashMap<String, (Token, bool, Vec<VariableModifier>)>;

pub type BoxedCall = Box<dyn Fn(Option<*mut dyn ExFnRef>, Vec<Token>) -> Result<Option<Token>, String>>;
pub type DynamicFunctionCall = (Option<*mut dyn ExFnRef>,
                                Option<BoxedCall>);
pub type RegisteredFunction= (String, DynamicFunctionCall);
pub type RegisteredVariable = (String, Token);

pub struct Runner
{
    pub parser: Parser,

    pub external_functions: HashMap<String, DynamicFunctionCall>,
    // bool - is external var
    // external vars cannot have their types changed.
    pub vars: VarMap,
    pub var_indexes_changed: Vec<String>,
}
