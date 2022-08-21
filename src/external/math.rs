use crate::types::{RegisteredFunction, RegisteredVariable};

use super::{FunctionCall, attach, register_function_return};

pub fn register_functs(functions: &mut Vec<RegisteredFunction>)
{
    register_function_return!(functions, "sqrt", f32::sqrt, f32, f32);
}

pub fn register_vars(_variables: &mut Vec<RegisteredVariable>)
{
}
