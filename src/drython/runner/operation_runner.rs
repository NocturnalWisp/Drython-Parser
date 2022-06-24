use std::collections::HashMap;

use crate::drython::types::{Token, Runner};

pub fn run_operation(operation: &Vec<Token>, parent_vars: &HashMap<String, Token>) -> Option<Token>
{
    let mut stack: Vec<Token> = vec![];
    for i in (0..operation.len()).rev()
    {
        

        // Push the last onto the stack.
        stack.push(operation[i].clone());
    }

    None
}

