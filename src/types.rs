use std::collections::HashMap;

#[derive(Debug)]
pub struct Func
{
    pub name: String,
    pub size: usize,
    pub parameters: Vec<String>,
    pub expressions: ExpressionList
}

#[derive(Debug)]
pub struct ExpressionList
{
    pub variables: HashMap<usize, (String, String)>,
    pub calls: HashMap<usize, (String, Vec<String>)>,
    pub internal_expressions: HashMap<usize, ExpressionList>,
    pub if_expression: Option<String>
}