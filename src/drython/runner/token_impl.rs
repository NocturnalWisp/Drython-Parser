use crate::drython::types::Token;

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
            _ => None
        }
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
            _ => None
        }
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
            _ => None
        }
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
            _ => None
        }
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
            _ => None
        }
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
            _ => None
        }
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
            _ => None
        }
    }

    pub fn check(&self) -> Option<Token>
    {
        match self
        {
            Token::Int(a) => Some(Token::Bool(*a != 0)),
            Token::Float(a) => Some(Token::Bool(*a != 0.0)),
            Token::Bool(a) => Some(Token::Bool(*a)),
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
            _ => None
        }
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
            _ => None
        }
    }

    pub fn compare_gte(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            (Token::Int(a), Token::Int(b)) => Some(Token::Bool(a >= b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Bool(a >= b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Bool((*a as f32) >= *b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Bool(*a >= (*b as f32))),
            _ => None
        }
    }

    pub fn compare_gt(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            (Token::Int(a), Token::Int(b)) => Some(Token::Bool(a > b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Bool(a > b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Bool((*a as f32) > *b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Bool(*a > (*b as f32))),
            _ => None
        }
    }

    pub fn compare_lte(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            (Token::Int(a), Token::Int(b)) => Some(Token::Bool(a <= b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Bool(a <= b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Bool((*a as f32) <= *b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Bool(*a <= (*b as f32))),
            _ => None
        }
    }

    pub fn compare_lt(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            (Token::Int(a), Token::Int(b)) => Some(Token::Bool(a < b)),
            (Token::Float(a), Token::Float(b)) => Some(Token::Bool(a < b)),
            (Token::Int(a), Token::Float(b)) => Some(Token::Bool((*a as f32) < *b)),
            (Token::Float(a), Token::Int(b)) => Some(Token::Bool(*a < (*b as f32))),
            _ => None
        }
    }
}