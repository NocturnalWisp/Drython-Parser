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

macro_rules! CollectionApply
{
    ($apply: ident, $name: ident) =>
    {
        fn $name(collection: &Vec<Token>, other_token: &Token) -> Token
        {
            Token::Collection(collection.iter().map(|x| x.$apply(other_token).unwrap_or(Token::Null)).collect())
        }
    };
}


macro_rules! CollectionApplyTogether
{
    ($apply: ident, $name: ident) =>
    {
        fn $name(collection: &Vec<Token>, other: &Vec<Token>) -> Token
        {
            if collection.len() == other.len()
            {
                let mut new_collection: Vec<Token> = Vec::new();

                for i in 0..collection.len()
                {
                    if let Some(applied) = collection[i].$apply(&other[i])
                    {
                        new_collection.push(applied);
                    }
                }

                return Token::Collection(new_collection);
            }

            Token::Null
        }
    };
}

#[allow(unused_variables)]
impl Token
{
    // Only check the variant and not the value.
    pub fn variant_equal(&self, other: &Token) -> bool
    {
        match (self, other)
        {
            (Token::Int(_), Token::Int(_)) => true,
            (Token::Float(_), Token::Float(_)) => true,
            (Token::String(_), Token::String(_)) => true,
            (Token::Char(_), Token::Char(_)) => true,
            (Token::Bool(_), Token::Bool(_)) => true,
            (Token::Collection(c1), Token::Collection(c2)) =>
            {
                if c1.len() != c2.len() { return false; }

                let mut all_equal = true;
                for i1 in c1.iter()
                {
                    for i2 in c2.iter()
                    {
                        if !Token::variant_equal(i1, i2)
                        {
                            all_equal = false;
                            break;
                        }
                    }
                }

                all_equal
            }
            _ => false
        }
    }

    pub fn add(&self, other: &Token) -> Option<Token>
    {
        match (self, other)
        {
            TIS!(Int, a, b) => Some(Token::Int(a + b)),
            TIS!(Float, a, b) => Some(Token::Float(*a as f32 + b)),
            TIS!(Bool, a, b) => Some(Token::Int(*a as i32 + *b as i32)),
            TIS!(Char, a, b) => Some(Token::String(format!("{}{}", a, b))),
            TIS!(String, a, b) => Some(Token::String(format!("{}{}", a, b))),

            TI!(Int, Float, a, b) =>
                Some(Token::Float(*a as f32 + b)),
            TI!(Int, Bool, a, b) =>
                Some(Token::Int(a + *b as i32)),
            TI!(Float, Bool, a, b) =>
                Some(Token::Float(a + (*b as i32) as f32)),

            TI!(String, Int, a, b) => Some(Token::String(format!("{}{}", a, b))),
            TI!(String, Float, a, b) => Some(Token::String(format!("{}{}", a, b))),
            TI!(String, Bool, a, b) => Some(Token::String(format!("{}{}", a, b))),
            TI!(String, Char, a, b) => Some(Token::String(format!("{}{}", a, b))),

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

            TIS!(Collection, a, b) => Some(Token::collection_add_together(a, b)),
            _ => None
        }
    }

    CollectionApply!(add, collection_add);
    CollectionApplyTogether!(add, collection_add_together);
    
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

            TIS!(Collection, a, b) => Some(Token::collection_subtract_together(a, b)),
            _ => None
        }
    }

    CollectionApply!(subtract, collection_subtract);
    CollectionApplyTogether!(subtract, collection_subtract_together);
    
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

            TIS!(Collection, a, b) => Some(Token::collection_multiply_together(a, b)),
            _ => None
        }
    }

    CollectionApply!(multiply, collection_multiply);
    CollectionApplyTogether!(multiply, collection_multiply_together);

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

            TIS!(Collection, a, b) => Some(Token::collection_divide_together(a, b)),
            _ => None
        }
    }

    CollectionApply!(divide, collection_divide);
    CollectionApplyTogether!(divide, collection_divide_together);

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

            TIS!(Collection, a, b) => Some(Token::collection_modulos_together(a, b)),
            _ => None
        }
    }

    CollectionApply!(modulos, collection_modulos);
    CollectionApplyTogether!(modulos, collection_modulos_together);

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

            TIS!(Collection, a, b) => Some(Token::collection_and_together(a, b)),
            _ => None
        }
    }

    CollectionApply!(and, collection_and);
    CollectionApplyTogether!(and, collection_and_together);

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

            TIS!(Collection, a, b) => Some(Token::collection_or_together(a, b)),
            _ => None
        }
    }

    CollectionApply!(or, collection_or);
    CollectionApplyTogether!(or, collection_or_together);

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

            TIS!(Collection, a, b) => Some(Token::collection_eq_together(a, b)),
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

    fn collection_eq_together(collection: &Vec<Token>, other: &Vec<Token>) -> Token
    {
        if collection.len() == other.len()
        {
            let mut all_equal = true;

            for i in 0..collection.len()
            {
                if let Some(Token::Bool(result)) = collection[i].compare_eq(&other[i])
                {
                    if !result
                    {
                        all_equal = false;
                        break;
                    }
                }
                else
                {
                    all_equal = false;
                    break;
                }
            }

            return Token::Bool(all_equal);
        }

        Token::Bool(false)
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

macro_rules! FromToToken
{
    ($t: ty, $token_type: tt) =>
    {
        impl From<$t> for Token
        {
            fn from(value: $t) -> Self
            {
                Token::$token_type(value)
            }
        }
    };
}

// From other types
FromToToken!(i32, Int);
FromToToken!(f32, Float);
FromToToken!(String, String);
FromToToken!(char, Char);
FromToToken!(bool, Bool);
FromToToken!(Vec<Token>, Collection);

macro_rules! TokenToOther
{
    ($token_type: tt, $t: ty, $default: expr) =>
    {
        impl From<Token> for $t
        {
            fn from(value: Token) -> Self
            {
                match value
                {
                    Token::$token_type(i) => i,
                    _ => $default
                }
            }
        }
    };
}

// Into other types
TokenToOther!(Int, i32, 0);
TokenToOther!(Float, f32, 0.0);
TokenToOther!(String, String, String::new());
TokenToOther!(Char, char, ' ');
TokenToOther!(Bool, bool, false);
TokenToOther!(Collection, Vec<Token>, vec![]);

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
