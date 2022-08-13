use std::collections::VecDeque;

pub struct ErrorManager
{
    pub errors: VecDeque<Box<dyn DrythonError>>
}

impl ErrorManager
{
    pub fn new() -> Self
    {
        ErrorManager
        {
            errors: VecDeque::new()
        }
    }

    pub fn add_error(&mut self, error: Box<dyn DrythonError>)
    {
        self.errors.push_back(error);
    }
}

// Use as a function on the error manager. (Eg. error_manager.error!(ParseError::new(0, 0, "Error.")); )
#[macro_export]
macro_rules! push_error
{
    ($manager: ident, $error: expr) =>
    {
        $manager.add_error(Box::new($error))
    }
}

pub (crate) use push_error;

// For parser
pub trait DrythonError
{
    fn display(&self) -> String
    {
        "".to_string()
    }
}

pub struct ParseError
{
    location: (usize, usize),
    message: String
}

impl ParseError
{
    pub fn new(line_number: usize, char_number: usize, message: &str) -> Self
    {
        ParseError
        {
            location: (line_number, char_number),
            message: message.to_string()
        }
    }
}
impl DrythonError for ParseError
{
    fn display(&self) -> String
    {
        format!("Drython Parse Error: Line [{}] Character [{}] - {}", self.location.0, self.location.1, self.message)
    }
}

pub struct ScriptTypeError
{
    first_line: String
}

impl ScriptTypeError
{
    pub fn new(first_line: String) -> Self
    {
        ScriptTypeError
        {
            first_line
        }
    }
}
impl DrythonError for ScriptTypeError
{
    fn display(&self) -> String
    {
        format!("Drython Unkown Script Type: {}. Please use a know script type such as \"Component\".", self.first_line)
    }
}

// For runner
pub struct RuntimeError
{
    line_number: usize,
    function_name: Option<String>,
    message: String
}

impl RuntimeError
{
    pub fn new(line_number: usize, function_name: Option<String>, message: &str) -> Self
    {
        RuntimeError
        {
            line_number,
            function_name,
            message: message.to_string()
        }
    }
}
impl DrythonError for RuntimeError
{
    fn display(&self) -> String
    {
        let mut in_function = false;
        if let Some(_) = self.function_name
        {
            in_function = true;
        }

        format!("Drython Runtime Error: {}Line [{}] - {}",
            if in_function { format!("Function [\"{}\"] ", self.function_name.clone().unwrap()) } else {"".to_string()},
            self.line_number,
            self.message
        )
    }
}
