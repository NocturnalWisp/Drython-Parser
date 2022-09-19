use crate::runner::VarMap;
use crate::runner::Token;
use crate::runner::VariableModifier;
use crate::runner::Runner;
use crate::types::SingleOp;

impl Runner
{
    pub fn handle_variable_modifiers(&mut self,
        variable: &SingleOp,
        operation_result: Token,
        modifier_list: Vec<VariableModifier>,
        is_scope: bool,
        scope_var_map: Option<&mut VarMap>) -> Result<(), String>
    {
        for modifier in &modifier_list
        {
            if let VariableModifier::Unkown(m) = modifier
            {
                return Err(format!("Unkown modifier '{}' on variable '{}'.", m, variable.0));
            }
            else if is_scope && !modifier.check_scope_allowed()
            {
                return Err(format!("Identifier '{:?}' is not allowed inside a scope. Place the variable definition outside a function.", modifier));
            }
        }

        if modifier_list.contains(&VariableModifier::Alias)
        {
            match &variable.2[0]
            {
                Token::Var(_) | Token::Call(_, _) =>
                {
                    if is_scope
                    {
                        scope_var_map.unwrap().insert(variable.0.to_string(), (variable.2[0].clone(), false, vec![VariableModifier::Alias]));
                        return Ok(());
                    }
                    else
                    {
                        self.vars.insert(variable.0.to_string(), (variable.2[0].clone(), false, vec![VariableModifier::Alias]));
                        return Ok(());
                    }
                }
                _ =>
                {
                    return Err(format!("Alias modifier expects a reference to another variable/function. Found: '{}'", variable.2[0]));
                }
            }
        }

        // External script reference.
        if modifier_list.contains(&VariableModifier::External)
        {
            if let Token::String(script_path) = &operation_result
            {
                if is_scope
                {
                    scope_var_map.unwrap().insert(variable.0.to_string(), (Token::, false, modifier_list));
                }
                else
                {
                    self.vars.insert(variable.0.to_string(), (operation_result, false, modifier_list));
                }
                return Ok(());
            }
            else
            {
                return Err(format!("External variables require a string path."));
            }
        }

        if is_scope
        {
            scope_var_map.unwrap().insert(variable.0.to_string(), (operation_result, false, modifier_list));
        }
        else
        {
            self.vars.insert(variable.0.to_string(), (operation_result, false, modifier_list));
        }

        Ok(())
    }
}
