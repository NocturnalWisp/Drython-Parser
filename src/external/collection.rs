use crate::types::{Token, RegisteredFunction, RegisteredVariable, ExFnRef};
use super::{expect, IsToken};

pub fn register_functs(functions: &mut Vec<RegisteredFunction>)
{
    functions.push(("push_collection".to_string(), (None, Some(Box::new(add_collection)))));
    functions.push(("remove_collection".to_string(), (None, Some(Box::new(remove_collection)))));
}

pub fn register_vars(_variables: &mut Vec<RegisteredVariable>)
{

}

fn add_collection(_: Option<*mut dyn ExFnRef>, mut args: Vec<Token>) -> Result<Option<Token>, String>
{
    match expect(&args, &[Some(&[IsToken::IsCollection]), None])
    {
        Err(error) => { return Err(error); }
        Ok(_) => ()
    }

    let mut new_collection: Vec<Token>;

    let new_token = args.pop();

    if let Some(Token::Collection(other)) = args.pop()
    {
        new_collection = other;
        new_collection.push(new_token.unwrap());
    }
    else
    {
        new_collection = vec![];
    }


    Ok(Some(Token::Collection(new_collection)))
}

fn remove_collection(_: Option<*mut dyn ExFnRef>, mut args: Vec<Token>) -> Result<Option<Token>, String>
{
    match expect(&args, &[Some(&[IsToken::IsCollection]),
        Some(&[IsToken::IsInt])])
    {
        Err(error) => { return Err(error); }
        Ok(_) => ()
    }

    let mut new_collection: Vec<Token>;

    let new_token = args.pop();

    match (args.pop(), new_token)
    {
        (Some(Token::Collection(other)), Some(Token::Int(i))) =>
        {
            if i > 0 && (i as usize) < other.len()
            {
                new_collection = other;
                new_collection.remove(i as usize);
            }
            else
            {
                return Err("Tried to remove from collection at index that does not exist.".to_string());
            }
        }
        // Should never happen, but needs to be covered for rust.
        _ => { new_collection = vec![]; }
    }

    Ok(Some(Token::Collection(new_collection)))
}
