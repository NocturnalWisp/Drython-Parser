mod expression_parser;
mod variable_parser;
pub mod operation_parser;

use std::fs;
use std::fmt::Write;
use crate::parser::expression_parser::parse_expressions;

use super::script_type::ScriptType;
use super::types::Parser;
use super::types::error::*;

#[derive(PartialEq, Debug, Clone)]
pub enum ExpressionType
{
    None,
    Assignment,
    Function,
    Call,
    Return,
    If,
    Elif,
    Else,
    Loop,
    Break,
    Continue,
    Library,
    Comment,
    End,
}

impl ExpressionType
{
    pub fn is_scope(&self) -> bool
    {
        match self
        {
            ExpressionType::Function => true,
            ExpressionType::If => true,
            ExpressionType::Loop => true,
            _ => false
        }
    }
}

impl Parser
{
    // Main parse function.
    pub fn parse_file(
        file_path: &str,
        error_manager: &mut ErrorManager
    ) -> Result<Parser, String>
    {
        let mut contents: String = match fs::read_to_string(&file_path)
        {
            Ok(string) => string,
            Err(_) => { return Err(format!("Error reading from file: {}", &file_path)); }
        };

        if contents.is_empty()
        {
            let mut s = String::new();
            write!(&mut s, "Contents of {} were empty.", file_path).ok();
            return Err(s);
        }

        // Allow multiple lines using '\'
        contents = contents.replace("\\\r\n", "");

        let lines: Vec<String> = 
            Parser::handle_content_replace(&contents, error_manager);

        // Determining Script Type.
        // let first_line = &lines[0][2..].trim_end_matches(";");
        // let script_type = ScriptType::from_string(first_line);

        // if let ScriptType::None = script_type
        // {
        //     return  Err(format!("Unexpected script type: {}", first_line));
        // }

        // Parse global expressions.
        let global_expressions = parse_expressions(&lines[0..].to_vec(), 0, error_manager, &ExpressionType::None, false);

        Ok(Parser
        {
            script_type: ScriptType::None,
            global_expressions,
        })
    }

    // Parses the content by removing empty spaces and placing semi colons at the end of lines.
    fn handle_content_replace(string: &str, error_manager: &mut ErrorManager) -> Vec<String>
    {
        let mut new_string: Vec<String> = Vec::new();

        let mut string_finished = false;

        for (index, line) in string.lines().enumerate()
        {
            // Remove all empty spaces except when in string literal.
            let mut new_line = String::new();
            let mut in_string_literal = false;

            for c in line.chars()
            {
                // Start new line line after semi-colons.
                if c == ';'
                {
                    new_string.push(format!("{}){}", index+1, new_line));
                    string_finished = true;

                    new_line = String::new();

                    if in_string_literal
                    {
                        push_error!(error_manager, ParseError::new(index, "String was not enclosed."));
                    }
                    in_string_literal = false;
                    continue;
                }

                string_finished = false;

                // Don't include white space unless in string literal.
                if (!c.is_whitespace()) || in_string_literal
                {
                    new_line.push(c);
                }

                if c == '"' || c == '\''
                {
                    in_string_literal = !in_string_literal;
                }
            }

            if !string_finished
            {
                new_string.push(format!("{}){}", index+1, new_line));
            }
        }
    
        new_string
    }
}
