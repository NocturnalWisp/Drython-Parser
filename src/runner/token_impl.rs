use crate::types::Token;
use std::{convert::From, fmt::{self, Display}};

// Match arms for most functions.
macro_rules! TI
{
    ($a: tt, $b: tt, $arg1: ident, $arg2: ident) =>
    {
        (Token::$a($arg1), Token::$b($arg2)) | (Token::$b($arg2), Token::$a($arg1))
    };
}

// Match arms for single arm in most functions.
macro_rules! TIS
{
    ($t: tt, $arg1: ident, $arg2: ident) =>
    {
        (Token::$t($arg1), Token::$t($arg2))
    };
    ($a: tt, $b: tt, $arg1: ident, $arg2: ident) =>
    {
        (Token::$a($arg1), Token::$b($arg2))
    };
}

#[allow(unused_variables)]
impl Token
{
    pub fn add(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            TIS!(Int, a, b) => Some(Token::Int(a + b)),
            TIS!(Float, a, b) => Some(Token::Float(*a as f32 + b)),
            TIS!(Bool, a, b) => Some(Token::Int(*a as i32 + *b as i32)),
            TIS!(Char, a, b) => Some(Token::String(format!("{}{}", a, b))),

            TI!(Int, Float, a, b) =>
                Some(Token::Float(*a as f32 + b)),
            TI!(Int, Bool, a, b) =>
                Some(Token::Int(a + *b as i32)),
            TI!(Float, Bool, a, b) =>
                Some(Token::Float(a + (*b as i32) as f32)),

            TI!(String, Int, a, b) => Some(Token::String(format!("{}{}", a, b))),
            TI!(String, Float, a, b) => Some(Token::String(format!("{}{}", a, b))),
            TI!(String, Bool, a, b) => Some(Token::String(format!("{}{}", a, b))),

            TI!(Char, Int, a, b) => Some(Token::String(format!("{}{}", a, b))),
            TI!(Char, Float, a, b) => Some(Token::String(format!("{}{}", a, b))),
            TI!(Char, Bool, a, b) => Some(Token::String(format!("{}{}", a, b))),

            TIS!(Collection, Int, a, b) => Some(Token::collection_add(a, other)),
            TIS!(Int, Collection, a, b) => Some(Token::collection_add(b, self)),

            TIS!(Collection, Float, a, b) => Some(Token::collection_add(a, other)),
            TIS!(Float, Collection, a, b) => Some(Token::collection_add(b, self)),

            TIS!(Collection, String, a, b) => Some(Token::collection_add(a, other)),
            TIS!(String, Collection, a, b) => Some(Token::collection_add(b, self)),

            TIS!(Collection, Bool, a, b) => Some(Token::collection_add(a, other)),
            TIS!(Bool, Collection, a, b) => Some(Token::collection_add(b, self)),
            _ => None
        }
    }

    fn collection_add(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Collection(collection.iter().map(|x| x.add(other_token).unwrap_or(Token::Null)).collect())
    }
    
    pub fn subtract(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            TIS!(Int, a, b) => Some(Token::Int(a - b)),
            TIS!(Float, a, b) => Some(Token::Float(a - b)),
            TIS!(Bool, a, b) => Some(Token::Int(*a as i32 - *b as i32)),

            TIS!(Int, Float, a, b) => Some(Token::Float(*a as f32 - b)),
            TIS!(Float, Int, a, b) => Some(Token::Float(a - *b as f32)),
            
            TIS!(Int, Bool, a, b) => Some(Token::Int(a - *b as i32)),
            TIS!(Bool, Int, a, b) => Some(Token::Int(*a as i32 - b)),

            TIS!(Float, Bool, a, b) => Some(Token::Float(a - (*b as i32) as f32)),
            TIS!(Bool, Float, a, b) => Some(Token::Float((*a as i32) as f32 - b)),
            
            TIS!(Collection, Int, a, b) => Some(Token::collection_subtract(a, other)),
            TIS!(Int, Collection, a, b) => Some(Token::collection_subtract(b, self)),

            TIS!(Collection, Float, a, b) => Some(Token::collection_subtract(a, other)),
            TIS!(Float, Collection, a, b) => Some(Token::collection_subtract(b, self)),
            _ => None
        }
    }

    fn collection_subtract(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Collection(collection.iter().map(|x| x.subtract(other_token).unwrap_or(Token::Null)).collect())
    }
    
    pub fn multiply(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            TIS!(Int, a, b) => Some(Token::Int(a * b)),
            TIS!(Float, a, b) => Some(Token::Float(*a as f32 * b)),
            TIS!(Bool, a, b) => Some(Token::Int(*a as i32 * *b as i32)),

            TI!(Int, Float, a, b) => Some(Token::Float(*a as f32 * b)),
            TI!(Int, Bool, a, b) => Some(Token::Int(a * *b as i32)),
            TI!(Float, Bool, a, b) => Some(Token::Float(a * (*b as i32) as f32)),
            
            TIS!(Collection, Int, a, b) => Some(Token::collection_multiply(a, other)),
            TIS!(Int, Collection, a, b) => Some(Token::collection_multiply(b, self)),

            TIS!(Collection, Float, a, b) => Some(Token::collection_multiply(a, other)),
            TIS!(Float, Collection, a, b) => Some(Token::collection_multiply(b, self)),
            _ => None
        }
    }

    fn collection_multiply(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Collection(collection.iter().map(|x| x.multiply(other_token).unwrap_or(Token::Null)).collect())
    }
    
    pub fn divide(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            TIS!(Int, a, b) => Some(Token::Int(a / b)),
            TIS!(Float, a, b) => Some(Token::Float(*a as f32 / b)),
            TIS!(Bool, a, b) => Some(Token::Int(*a as i32 / *b as i32)),

            TIS!(Int, Float, a, b) => Some(Token::Float(*a as f32 / b)),
            TIS!(Float, Int, a, b) => Some(Token::Float(a / *b as f32)),
            
            TIS!(Int, Bool, a, b) => Some(Token::Int(a / *b as i32)),
            TIS!(Bool, Int, a, b) => Some(Token::Int(*a as i32 / b)),

            TIS!(Float, Bool, a, b) => Some(Token::Float(a / (*b as i32) as f32)),
            TIS!(Bool, Float, a, b) => Some(Token::Float((*a as i32) as f32 / b)),
           
            TIS!(Collection, Int, a, b) => Some(Token::collection_divide(a, other)),
            TIS!(Int, Collection, a, b) => Some(Token::collection_divide(b, self)),

            TIS!(Collection, Float, a, b) => Some(Token::collection_divide(a, other)),
            TIS!(Float, Collection, a, b) => Some(Token::collection_divide(b, self)),
            _ => None
        }
    }

    fn collection_divide(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Collection(collection.iter().map(|x| x.divide(other_token).unwrap_or(Token::Null)).collect())
    }
    
    pub fn modulos(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            TIS!(Int, a, b) => Some(Token::Int(a % b)),
            TIS!(Float, a, b) => Some(Token::Float(*a as f32 % b)),
            TIS!(Bool, a, b) => Some(Token::Int(*a as i32 % *b as i32)),

            TIS!(Int, Float, a, b) => Some(Token::Float(*a as f32 % b)),
            TIS!(Float, Int, a, b) => Some(Token::Float(a % *b as f32)),
            
            TIS!(Int, Bool, a, b) => Some(Token::Int(a % *b as i32)),
            TIS!(Bool, Int, a, b) => Some(Token::Int(*a as i32 % b)),

            TIS!(Float, Bool, a, b) => Some(Token::Float(a % (*b as i32) as f32)),
            TIS!(Bool, Float, a, b) => Some(Token::Float((*a as i32) as f32 % b)),
            
            TIS!(Collection, Int, a, b) => Some(Token::collection_modulos(a, other)),
            TIS!(Int, Collection, a, b) => Some(Token::collection_modulos(b, self)),

            TIS!(Collection, Float, a, b) => Some(Token::collection_modulos(a, other)),
            TIS!(Float, Collection, a, b) => Some(Token::collection_modulos(b, self)),
            _ => None
        }
    }

    fn collection_modulos(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Collection(collection.iter().map(|x| x.modulos(other_token).unwrap_or(Token::Null)).collect())
    }
    
    pub fn and(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            TIS!(Int, a, b) => Some(Token::Bool(*a != 0 && *b != 0)),
            TIS!(Float, a, b) => Some(Token::Bool(*a != 0.0 && *b != 0.0)),
            TIS!(Bool, a, b) => Some(Token::Bool(*a && *b)),

            TI!(Int, Float, a, b) => Some(Token::Bool(*a != 0 && *b != 0.0)),
            
            TI!(Int, Bool, a, b) => Some(Token::Bool(*a != 0 && *b)),

            TI!(Float, Bool, a, b) => Some(Token::Bool(*a != 0.0 && *b)),
            
            TIS!(Collection, Int, a, b) => Some(Token::collection_and(a, other)),
            TIS!(Int, Collection, a, b) => Some(Token::collection_and(b, self)),

            TIS!(Collection, Float, a, b) => Some(Token::collection_and(a, other)),
            TIS!(Float, Collection, a, b) => Some(Token::collection_and(b, self)),

            TIS!(Collection, Bool, a, b) => Some(Token::collection_and(a, other)),
            TIS!(Bool, Collection, a, b) => Some(Token::collection_and(b, self)),
            _ => None
        }
    }

    fn collection_and(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Collection(collection.iter().map(|x| x.and(other_token).unwrap_or(Token::Null)).collect())
    }

    pub fn or(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            TIS!(Int, a, b) => Some(Token::Bool(*a != 0 || *b != 0)),
            TIS!(Float, a, b) => Some(Token::Bool(*a != 0.0 || *b != 0.0)),
            TIS!(Bool, a, b) => Some(Token::Bool(*a || *b)),

            TI!(Float, Int, a, b) => Some(Token::Bool(*a != 0.0 || *b != 0)),
            
            TI!(Bool, Int, a, b) => Some(Token::Bool(*a || *b != 0)),

            TI!(Bool, Float, a, b) => Some(Token::Bool(*a || *b != 0.0)),
            
            TIS!(Collection, Int, a, b) => Some(Token::collection_or(a, other)),
            TIS!(Int, Collection, a, b) => Some(Token::collection_or(b, self)),

            TIS!(Collection, Float, a, b) => Some(Token::collection_or(a, other)),
            TIS!(Float, Collection, a, b) => Some(Token::collection_or(b, self)),

            TIS!(Collection, Bool, a, b) => Some(Token::collection_or(a, other)),
            TIS!(Bool, Collection, a, b) => Some(Token::collection_or(b, self)),
            _ => None
        }
    }

    fn collection_or(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Collection(collection.iter().map(|x| x.or(other_token).unwrap_or(Token::Null)).collect())
    }

    pub fn check(&self) -> Option<Token>
    {
        match self
        {
            Token::Int(a) => Some(Token::Bool(*a != 0)),
            Token::Float(a) => Some(Token::Bool(*a != 0.0)),
            Token::Bool(a) => Some(Token::Bool(*a)),
            
            Token::Collection(a) => Some(Token::Collection(a.iter().map(|x| x.check().unwrap_or(Token::Null)).collect())),
            _ => None
        }
    }

    pub fn compare_eq(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            TIS!(Int, a, b) => Some(Token::Bool(a == b)),
            TIS!(Float, a, b) => Some(Token::Bool(a == b)),
            TIS!(Bool, a, b) => Some(Token::Bool(*a == *b)),
            TIS!(String, a, b) => Some(Token::Bool(a == b)),
            TIS!(Char, a, b) => Some(Token::Bool(a == b)),

            TI!(Int, Float, a, b) => Some(Token::Bool((*a as f32) == *b)),
            TI!(Int, Char, a, b) => Some(Token::Bool(a.to_string() == b.to_string())),
            TI!(Int, Bool, a, b) => Some(Token::Bool((*a != 0) == *b)),
            TI!(Float, Bool, a, b) => Some(Token::Bool((*a != 0.0) == *b)),

            TI!(String, Char, a, b) => Some(Token::Bool(*a == b.to_string())),

            TI!(String, Int, a, b) => Some(Token::Bool(*a == b.to_string())),
            TI!(String, Float, a, b) => Some(Token::Bool(*a == b.to_string())),
            TI!(String, Bool, a, b) => Some(Token::Bool(*a == b.to_string())),
            
            TIS!(Collection, Int, a, b) => Some(Token::collection_eq(a, other)),
            TIS!(Collection, Float, a, b) => Some(Token::collection_eq(a, other)),
            TIS!(Collection, Bool, a, b) => Some(Token::collection_eq(a, other)),
            TIS!(Collection, String, a, b) => Some(Token::collection_eq(a, other)),
            TIS!(Collection, Char, a, b) => Some(Token::collection_eq(a, other)),
            TIS!(Int, Collection, a, b) => Some(Token::collection_eq(b, self)),
            TIS!(Float, Collection, a, b) => Some(Token::collection_eq(b, self)),
            TIS!(Bool, Collection, a, b) => Some(Token::collection_eq(b, self)),
            TIS!(String, Collection, a, b) => Some(Token::collection_eq(b, self)),
            TIS!(Char, Collection, a, b) => Some(Token::collection_eq(b, self)),
            _ => None
        }
    }

    fn collection_eq(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Bool(collection.iter().all(|x| 
                                            if let Some(Token::Bool(true)) = x.compare_eq(other_token)
                                            {
                                                true
                                            } else {false}))
    }

    pub fn compare_neq(&self, other: &Token) -> Option<Token>
    {
        if let Some(Token::Bool(value)) = self.compare_eq(other)
        {
            Some(Token::Bool(!value))
        }
        else
        {
            None
        }
    }

    pub fn compare_gte(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            TIS!(Int, a, b) => Some(Token::Bool(a >= b)),
            TIS!(Float, a, b) => Some(Token::Bool(a >= b)),
            TI!(Int, Float, a, b) => Some(Token::Bool((*a as f32) >= *b)),
            
            TIS!(Collection, Int, a, b) => Some(Token::collection_gte(a, other)),
            TIS!(Int, Collection, a, b) => Some(Token::collection_gte(b, self)),

            TIS!(Collection, Float, a, b) => Some(Token::collection_gte(a, other)),
            TIS!(Float, Collection, a, b) => Some(Token::collection_gte(b, self)),
            _ => None
        }
    }

    fn collection_gte(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Bool(collection.iter().all(|x| 
                                            if let Some(Token::Bool(true)) = x.compare_gte(other_token)
                                            {
                                                true
                                            } else {false}))
    }

    // Opposite of gte
    pub fn compare_lt(&self, other: &Token) -> Option<Token>
    {
        if let Some(Token::Bool(value)) = self.compare_gte(other)
        {
            Some(Token::Bool(!value))
        }
        else
        {
            None
        }
    }

    pub fn compare_gt(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            TIS!(Int, a, b) => Some(Token::Bool(a > b)),
            TIS!(Float, a, b) => Some(Token::Bool(a > b)),
            TI!(Int, Float, a, b) => Some(Token::Bool((*a as f32) > *b)),
            
            TIS!(Collection, Int, a, b) => Some(Token::collection_gt(a, other)),
            TIS!(Int, Collection, a, b) => Some(Token::collection_gt(b, self)),

            TIS!(Collection, Float, a, b) => Some(Token::collection_gt(a, other)),
            TIS!(Float, Collection, a, b) => Some(Token::collection_gt(b, self)),
            _ => None
        }
    }

    fn collection_gt(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Bool(collection.iter().all(|x| 
                                            if let Some(Token::Bool(true)) = x.compare_gt(other_token)
                                            {
                                                true
                                            } else {false}))
    }

    // Opposite of gt
    pub fn compare_lte(&self, other: &Token) -> Option<Token>
    {
        if let Some(Token::Bool(value)) = self.compare_gt(other)
        {
            Some(Token::Bool(!value))
        }
        else
        {
            None
        }
    }
}

// From other types
impl From<i32> for Token
{
    fn from(value: i32) -> Self
    {
        Token::Int(value)
    }
}

impl From<f32> for Token
{
    fn from(value: f32) -> Self
    {
        Token::Float(value)
    }
}

impl From<bool> for Token
{
    fn from(value: bool) -> Self
    {
        Token::Bool(value)
    }
}

impl From<String> for Token
{
    fn from(value: String) -> Self
    {
        Token::String(value)
    }
}

impl From<char> for Token
{
    fn from(value: char) -> Self
    {
        Token::Char(value)
    }
}

impl From<Vec<Token>> for Token
{
    fn from(value: Vec<Token>) -> Self
    {
        Token::Collection(value)
    }
}

// Into other types
impl From<Token> for i32
{
    fn from(value: Token) -> Self
    {
        match value
        {
            Token::Int(i) => i,
            _ => 0
        }
    }
}

impl From<Token> for f32
{
    fn from(value: Token) -> Self
    {
        match value
        {
            Token::Float(i) => i,
            _ => 0.0
        }
    }
}

impl From<Token> for String
{
    fn from(value: Token) -> Self
    {
        match value
        {
            Token::String(i) => i,
            _ => "".to_string()
        }
    }
}

impl From<Token> for char
{
    fn from(value: Token) -> Self
    {
        match value
        {
            Token::Char(c) => c,
            _ => ' '
        }
    }
}

impl From<Token> for bool
{
    fn from(value: Token) -> Self
    {
        match value
        {
            Token::Bool(i) => i,
            _ => false
        }
    }
}

impl Display for Token
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", match self
        {
            Token::Int(i) => i.to_string(),
            Token::Float(f) => f.to_string(),
            Token::Bool(b) => b.to_string(),
            Token::String(s) => format!("\"{}\"", s),
            Token::Char(c) => format!("'{}'", c),
            Token::Collection(c) =>
            {
                let mut s = String::new();
                s.push('[');
                for token in c
                {
                    s.push_str(&token.to_string());
                    s.push(',');
                }
                s.pop();
                s.push(']');
                s
            },
            _ => "".to_string()
        })
    }
}
