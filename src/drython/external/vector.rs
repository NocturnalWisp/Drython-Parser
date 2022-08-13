use crate::drython::types::{Token::{self, *}, RegisteredFunction, RegisteredVariable};

use super::{FunctionCall, attach, register_function_return};

pub fn register_functs(functions: &mut Vec<RegisteredFunction>)
{
    register_function_return!(functions, "divide2", divide2, Vec<f32>, Vec<f32>);
}

pub fn register_vars(variables: &mut Vec<RegisteredVariable>)
{
    variables.push(("vector3.one".to_string(), Collection(vec![Float(1.0), Float(1.0), Float(1.0)])));
}

impl From<Vec<f32>> for Token
{
    fn from(value: Vec<f32>) -> Self
    {
        let mut tokens: Vec<Token> = vec![];
        for f in value
        {
            tokens.push(Token::Float(f));
        }

        Token::Collection(tokens)
    }
}

impl From<Token> for Vec<f32>
{
    fn from(value: Token) -> Self
    {
        match &value
        {
            Token::Collection(collection) =>
            {
                let mut vector: Vec<f32> = vec![];
                for item in collection
                {
                    if let Token::Float(f) = item
                    {
                        vector.push(*f);
                    }
                    else
                    {
                        return vec![];
                    }
                }

                vector
            },
            _ => vec![]
        }
    }
}

fn divide2(vector: Vec<f32>) -> Vec<f32>
{
    return vector.iter().map(|f| *f/2.0).collect::<Vec<f32>>();
}
