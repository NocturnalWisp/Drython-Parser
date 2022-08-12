use crate::drython::types::{Token, RegisteredFunction, RegisteredVariable};
use super::IsToken::{*};

use super::{expect, FunctionCall, attach, register_function, register_function_return};

pub fn register_functs(functions: &mut Vec<RegisteredFunction>)
{
    register_function_return!(functions, "sqrt", f32::sqrt, f32, f32);
}

pub fn register_vars(variables: &mut Vec<RegisteredVariable>)
{
}
