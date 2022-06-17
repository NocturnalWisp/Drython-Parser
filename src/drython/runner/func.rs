use crate::drython::types::ExpressionList;
use crate::drython::types::RegisteredList;

use super::var;

// pub fn check_func<'a>(func: &'a Func, registered: &mut RegisteredList<'a>) -> Result<(), String>
// {
//     if !func.name.chars().all(|x| x.is_alphanumeric())
//     {
//         return Err("The function's name cannot have special characters.".to_string());
//     }

//     registered.func_list.push(&func.name);

//     check_expression(&func.expressions, func.size, registered);

//     Ok(())
// }

fn check_expression<'a>(
    expressions: &'a ExpressionList,
    expression_size: usize,
    registered: &mut RegisteredList<'a>)
{
    for index in 0..expression_size
    {
        // Check variable.
        if expressions.variables.contains_key(&index)
        {
            //let var = &expressions.variables[&index];
            //var::check_var((&var.0.as_str(), &var.1.as_str()), registered);
        }
    }
}