use std::collections::HashMap;
use super::Token;

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
