use crate::drython::types::Token;
use std::convert::From;

impl Token
{
    pub fn add(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            (Token::Int(a), Token::Int(b)) => Some(Token::Int(a + b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Float(*a as f32 + b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Float(*a as f32 + b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Float(a + *b as f32)),
            
            (Token::Bool(a), Token::Bool(b)) => Some(Token::Int(*a as i32 + *b as i32)),
            (Token::Int(a), Token::Bool(b)) => Some(Token::Int(a + *b as i32)),
            (Token::Float(a), Token::Bool(b)) => Some(Token::Float(a + (*b as i32) as f32)),
            (Token::Bool(a), Token::Int(b)) => Some(Token::Int(*a as i32 + b)),
            (Token::Bool(a), Token::Float(b)) => Some(Token::Float((*a as i32) as f32 + b)),

            (Token::String(a), Token::Int(b)) => Some(Token::String(format!("{}{}", a, b))),
            (Token::String(a), Token::Float(b)) => Some(Token::String(format!("{}{}", a, b))),
            (Token::String(a), Token::Bool(b)) => Some(Token::String(format!("{}{}", a, b))),

            (Token::String(a), Token::String(b)) => Some(Token::String(format!("{}{}", a, b))),
            (Token::Int(a), Token::String(b)) => Some(Token::String(format!("{}{}", a, b))),
            (Token::Float(a), Token::String(b)) => Some(Token::String(format!("{}{}", a, b))),
            (Token::Bool(a), Token::String(b)) => Some(Token::String(format!("{}{}", a, b))),
            
            (Token::Collection(a), Token::Int(_)) => Some(Token::collection_add(a, other)),
            (Token::Collection(a), Token::Float(_)) => Some(Token::collection_add(a, other)),
            (Token::Collection(a), Token::String(_)) => Some(Token::collection_add(a, other)),
            (Token::Int(_), Token::Collection(b)) => Some(Token::collection_add(b, self)),
            (Token::Float(_), Token::Collection(b)) => Some(Token::collection_add(b, self)),
            (Token::String(_), Token::Collection(b)) => Some(Token::collection_add(b, self)),
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
            (Token::Int(a), Token::Int(b)) => Some(Token::Int(a - b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Float(*a as f32 - b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Float(*a as f32 - b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Float(a - *b as f32)),
            
            (Token::Bool(a), Token::Bool(b)) => Some(Token::Int(*a as i32 - *b as i32)),
            (Token::Int(a), Token::Bool(b)) => Some(Token::Int(a - *b as i32)),
            (Token::Float(a), Token::Bool(b)) => Some(Token::Float(a - (*b as i32) as f32)),
            (Token::Bool(a), Token::Int(b)) => Some(Token::Int(*a as i32 - b)),
            (Token::Bool(a), Token::Float(b)) => Some(Token::Float((*a as i32) as f32 - b)),
            
            (Token::Collection(a), Token::Int(_)) => Some(Token::collection_subtract(a, other)),
            (Token::Collection(a), Token::Float(_)) => Some(Token::collection_subtract(a, other)),
            (Token::Collection(a), Token::String(_)) => Some(Token::collection_subtract(a, other)),
            (Token::Int(_), Token::Collection(b)) => Some(Token::collection_subtract(b, self)),
            (Token::Float(_), Token::Collection(b)) => Some(Token::collection_subtract(b, self)),
            (Token::String(_), Token::Collection(b)) => Some(Token::collection_subtract(b, self)),
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
            (Token::Int(a), Token::Int(b)) => Some(Token::Int(a * b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Float(*a as f32 * b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Float(*a as f32 * b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Float(a * *b as f32)),
            
            (Token::Bool(a), Token::Bool(b)) => Some(Token::Int(*a as i32 * *b as i32)),
            (Token::Int(a), Token::Bool(b)) => Some(Token::Int(a * *b as i32)),
            (Token::Float(a), Token::Bool(b)) => Some(Token::Float(a * (*b as i32) as f32)),
            (Token::Bool(a), Token::Int(b)) => Some(Token::Int(*a as i32 * b)),
            (Token::Bool(a), Token::Float(b)) => Some(Token::Float((*a as i32) as f32 * b)),
            
            (Token::Collection(a), Token::Int(_)) => Some(Token::collection_multiply(a, other)),
            (Token::Collection(a), Token::Float(_)) => Some(Token::collection_multiply(a, other)),
            (Token::Collection(a), Token::String(_)) => Some(Token::collection_multiply(a, other)),
            (Token::Int(_), Token::Collection(b)) => Some(Token::collection_multiply(b, self)),
            (Token::Float(_), Token::Collection(b)) => Some(Token::collection_multiply(b, self)),
            (Token::String(_), Token::Collection(b)) => Some(Token::collection_multiply(b, self)),
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
            (Token::Int(a), Token::Int(b)) => Some(Token::Int(a / b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Float(*a as f32 / b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Float(*a as f32 / b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Float(a / *b as f32)),
            
            (Token::Bool(a), Token::Bool(b)) => Some(Token::Int(*a as i32 / *b as i32)),
            (Token::Int(a), Token::Bool(b)) => Some(Token::Int(a / *b as i32)),
            (Token::Float(a), Token::Bool(b)) => Some(Token::Float(a / (*b as i32) as f32)),
            (Token::Bool(a), Token::Int(b)) => Some(Token::Int(*a as i32 / b)),
            (Token::Bool(a), Token::Float(b)) => Some(Token::Float((*a as i32) as f32 / b)),
           
            (Token::Collection(a), Token::Int(_)) => Some(Token::collection_divide(a, other)),
            (Token::Collection(a), Token::Float(_)) => Some(Token::collection_divide(a, other)),
            (Token::Collection(a), Token::String(_)) => Some(Token::collection_divide(a, other)),
            (Token::Int(_), Token::Collection(b)) => Some(Token::collection_divide(b, self)),
            (Token::Float(_), Token::Collection(b)) => Some(Token::collection_divide(b, self)),
            (Token::String(_), Token::Collection(b)) => Some(Token::collection_divide(b, self)),
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
            (Token::Int(a), Token::Int(b)) => Some(Token::Int(a % b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Float(*a as f32 % b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Float(*a as f32 % b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Float(a % *b as f32)),
            
            (Token::Bool(a), Token::Bool(b)) => Some(Token::Int(*a as i32 % *b as i32)),
            (Token::Int(a), Token::Bool(b)) => Some(Token::Int(a % *b as i32)),
            (Token::Float(a), Token::Bool(b)) => Some(Token::Float(a % (*b as i32) as f32)),
            (Token::Bool(a), Token::Int(b)) => Some(Token::Int(*a as i32 % b)),
            (Token::Bool(a), Token::Float(b)) => Some(Token::Float((*a as i32) as f32 % b)),
            
            (Token::Collection(a), Token::Int(_)) => Some(Token::collection_modulos(a, other)),
            (Token::Collection(a), Token::Float(_)) => Some(Token::collection_modulos(a, other)),
            (Token::Collection(a), Token::String(_)) => Some(Token::collection_modulos(a, other)),
            (Token::Int(_), Token::Collection(b)) => Some(Token::collection_modulos(b, self)),
            (Token::Float(_), Token::Collection(b)) => Some(Token::collection_modulos(b, self)),
            (Token::String(_), Token::Collection(b)) => Some(Token::collection_modulos(b, self)),
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
            (Token::Int(a), Token::Int(b)) => Some(Token::Bool(*a != 0 && *b != 0)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Bool(*a != 0.0 && *b != 0.0)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Bool(*a != 0 && *b != 0.0)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Bool(*a != 0.0 && *b != 0)),
            
            (Token::Bool(a), Token::Bool(b)) => Some(Token::Bool(*a && *b)),
            (Token::Int(a), Token::Bool(b)) => Some(Token::Bool(*a != 0 && *b)),
            (Token::Float(a), Token::Bool(b)) => Some(Token::Bool(*a != 0.0 && *b)),
            (Token::Bool(a), Token::Int(b)) => Some(Token::Bool(*a && *b != 0)),
            (Token::Bool(a), Token::Float(b)) => Some(Token::Bool(*a && *b != 0.0)),
            
            (Token::Collection(a), Token::Int(_)) => Some(Token::collection_and(a, other)),
            (Token::Collection(a), Token::Float(_)) => Some(Token::collection_and(a, other)),
            (Token::Collection(a), Token::Bool(_)) => Some(Token::collection_and(a, other)),
            (Token::Collection(a), Token::String(_)) => Some(Token::collection_and(a, other)),
            (Token::Int(_), Token::Collection(b)) => Some(Token::collection_and(b, self)),
            (Token::Float(_), Token::Collection(b)) => Some(Token::collection_and(b, self)),
            (Token::String(_), Token::Collection(b)) => Some(Token::collection_and(b, self)),
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
            (Token::Int(a), Token::Int(b)) => Some(Token::Bool(*a != 0 || *b != 0)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Bool(*a != 0.0 || *b != 0.0)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Bool(*a != 0 || *b != 0.0)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Bool(*a != 0.0 || *b != 0)),
            
            (Token::Bool(a), Token::Bool(b)) => Some(Token::Bool(*a || *b)),
            (Token::Int(a), Token::Bool(b)) => Some(Token::Bool(*a != 0 || *b)),
            (Token::Float(a), Token::Bool(b)) => Some(Token::Bool(*a != 0.0 || *b)),
            (Token::Bool(a), Token::Int(b)) => Some(Token::Bool(*a || *b != 0)),
            (Token::Bool(a), Token::Float(b)) => Some(Token::Bool(*a || *b != 0.0)),
            
            (Token::Collection(a), Token::Int(_)) => Some(Token::collection_or(a, other)),
            (Token::Collection(a), Token::Float(_)) => Some(Token::collection_or(a, other)),
            (Token::Collection(a), Token::Bool(_)) => Some(Token::collection_or(a, other)),
            (Token::Collection(a), Token::String(_)) => Some(Token::collection_or(a, other)),
            (Token::Int(_), Token::Collection(b)) => Some(Token::collection_or(b, self)),
            (Token::Float(_), Token::Collection(b)) => Some(Token::collection_or(b, self)),
            (Token::Bool(_), Token::Collection(b)) => Some(Token::collection_or(b, self)),
            (Token::String(_), Token::Collection(b)) => Some(Token::collection_or(b, self)),
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
            (Token::Int(a), Token::Int(b)) => Some(Token::Bool(a == b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Bool(a == b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Bool((*a as f32) == *b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Bool(*a == (*b as f32))),
            
            (Token::Bool(a), Token::Bool(b)) => Some(Token::Bool(*a == *b)),
            (Token::Int(a), Token::Bool(b)) => Some(Token::Bool((*a != 0) == *b)),
            (Token::Float(a), Token::Bool(b)) => Some(Token::Bool((*a != 0.0) == *b)),
            (Token::Bool(a), Token::Int(b)) => Some(Token::Bool(*a == (*b != 0))),
            (Token::Bool(a), Token::Float(b)) => Some(Token::Bool(*a == (*b != 0.0))),
            
            (Token::Collection(a), Token::Int(_)) => Some(Token::collection_eq(a, other)),
            (Token::Collection(a), Token::Float(_)) => Some(Token::collection_eq(a, other)),
            (Token::Collection(a), Token::Bool(_)) => Some(Token::collection_eq(a, other)),
            (Token::Collection(a), Token::String(_)) => Some(Token::collection_eq(a, other)),
            (Token::Int(_), Token::Collection(b)) => Some(Token::collection_eq(b, self)),
            (Token::Float(_), Token::Collection(b)) => Some(Token::collection_eq(b, self)),
            (Token::Bool(_), Token::Collection(b)) => Some(Token::collection_eq(b, self)),
            (Token::String(_), Token::Collection(b)) => Some(Token::collection_eq(b, self)),
            _ => None
        }
    }

    fn collection_eq(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Collection(collection.iter().map(|x| x.compare_eq(other_token).unwrap_or(Token::Null)).collect())
    }

    pub fn compare_neq(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            (Token::Int(a), Token::Int(b)) => Some(Token::Bool(a != b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Bool(a != b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Bool((*a as f32) != *b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Bool(*a != (*b as f32))),
            
            (Token::Bool(a), Token::Bool(b)) => Some(Token::Bool(*a != *b)),
            (Token::Int(a), Token::Bool(b)) => Some(Token::Bool((*a != 0) != *b)),
            (Token::Float(a), Token::Bool(b)) => Some(Token::Bool((*a != 0.0) != *b)),
            (Token::Bool(a), Token::Int(b)) => Some(Token::Bool(*a != (*b != 0))),
            (Token::Bool(a), Token::Float(b)) => Some(Token::Bool(*a != (*b != 0.0))),
            
            (Token::Collection(a), Token::Int(_)) => Some(Token::collection_neq(a, other)),
            (Token::Collection(a), Token::Float(_)) => Some(Token::collection_neq(a, other)),
            (Token::Collection(a), Token::Bool(_)) => Some(Token::collection_neq(a, other)),
            (Token::Collection(a), Token::String(_)) => Some(Token::collection_neq(a, other)),
            (Token::Int(_), Token::Collection(b)) => Some(Token::collection_neq(b, self)),
            (Token::Float(_), Token::Collection(b)) => Some(Token::collection_neq(b, self)),
            (Token::Bool(_), Token::Collection(b)) => Some(Token::collection_neq(b, self)),
            (Token::String(_), Token::Collection(b)) => Some(Token::collection_neq(b, self)),
            _ => None
        }
    }

    fn collection_neq(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Collection(collection.iter().map(|x| x.compare_neq(other_token).unwrap_or(Token::Null)).collect())
    }

    pub fn compare_gte(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            (Token::Int(a), Token::Int(b)) => Some(Token::Bool(a >= b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Bool(a >= b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Bool((*a as f32) >= *b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Bool(*a >= (*b as f32))),
            
            (Token::Collection(a), Token::Int(_)) => Some(Token::collection_gte(a, other)),
            (Token::Collection(a), Token::Float(_)) => Some(Token::collection_gte(a, other)),
            (Token::Collection(a), Token::Bool(_)) => Some(Token::collection_gte(a, other)),
            (Token::Collection(a), Token::String(_)) => Some(Token::collection_gte(a, other)),
            (Token::Int(_), Token::Collection(b)) => Some(Token::collection_gte(b, self)),
            (Token::Float(_), Token::Collection(b)) => Some(Token::collection_gte(b, self)),
            (Token::Bool(_), Token::Collection(b)) => Some(Token::collection_gte(b, self)),
            (Token::String(_), Token::Collection(b)) => Some(Token::collection_gte(b, self)),
            _ => None
        }
    }

    fn collection_gte(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Collection(collection.iter().map(|x| x.compare_gte(other_token).unwrap_or(Token::Null)).collect())
    }

    pub fn compare_gt(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            (Token::Int(a), Token::Int(b)) => Some(Token::Bool(a > b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Bool(a > b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Bool((*a as f32) > *b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Bool(*a > (*b as f32))),
            
            (Token::Collection(a), Token::Int(_)) => Some(Token::collection_gt(a, other)),
            (Token::Collection(a), Token::Float(_)) => Some(Token::collection_gt(a, other)),
            (Token::Collection(a), Token::Bool(_)) => Some(Token::collection_gt(a, other)),
            (Token::Collection(a), Token::String(_)) => Some(Token::collection_gt(a, other)),
            (Token::Int(_), Token::Collection(b)) => Some(Token::collection_gt(b, self)),
            (Token::Float(_), Token::Collection(b)) => Some(Token::collection_gt(b, self)),
            (Token::Bool(_), Token::Collection(b)) => Some(Token::collection_gt(b, self)),
            (Token::String(_), Token::Collection(b)) => Some(Token::collection_gt(b, self)),
            _ => None
        }
    }

    fn collection_gt(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Collection(collection.iter().map(|x| x.compare_gt(other_token).unwrap_or(Token::Null)).collect())
    }

    pub fn compare_lte(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            (Token::Int(a), Token::Int(b)) => Some(Token::Bool(a <= b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Bool(a <= b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Bool((*a as f32) <= *b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Bool(*a <= (*b as f32))),
            
            (Token::Collection(a), Token::Int(_)) => Some(Token::collection_lte(a, other)),
            (Token::Collection(a), Token::Float(_)) => Some(Token::collection_lte(a, other)),
            (Token::Collection(a), Token::Bool(_)) => Some(Token::collection_lte(a, other)),
            (Token::Collection(a), Token::String(_)) => Some(Token::collection_lte(a, other)),
            (Token::Int(_), Token::Collection(b)) => Some(Token::collection_lte(b, self)),
            (Token::Float(_), Token::Collection(b)) => Some(Token::collection_lte(b, self)),
            (Token::Bool(_), Token::Collection(b)) => Some(Token::collection_lte(b, self)),
            (Token::String(_), Token::Collection(b)) => Some(Token::collection_lte(b, self)),
            _ => None
        }
    }

    fn collection_lte(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Collection(collection.iter().map(|x| x.compare_lte(other_token).unwrap_or(Token::Null)).collect())
    }

    pub fn compare_lt(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            (Token::Int(a), Token::Int(b)) => Some(Token::Bool(a < b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Bool(a < b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Bool((*a as f32) < *b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Bool(*a < (*b as f32))),
            
            (Token::Collection(a), Token::Int(_)) => Some(Token::collection_lt(a, other)),
            (Token::Collection(a), Token::Float(_)) => Some(Token::collection_lt(a, other)),
            (Token::Collection(a), Token::Bool(_)) => Some(Token::collection_lt(a, other)),
            (Token::Collection(a), Token::String(_)) => Some(Token::collection_lt(a, other)),
            (Token::Int(_), Token::Collection(b)) => Some(Token::collection_lt(b, self)),
            (Token::Float(_), Token::Collection(b)) => Some(Token::collection_lt(b, self)),
            (Token::Bool(_), Token::Collection(b)) => Some(Token::collection_lt(b, self)),
            (Token::String(_), Token::Collection(b)) => Some(Token::collection_lt(b, self)),
            _ => None
        }
    }

    fn collection_lt(collection: &Vec<Token>, other_token: &Token) -> Token
    {
        Token::Collection(collection.iter().map(|x| x.compare_lt(other_token).unwrap_or(Token::Null)).collect())
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
