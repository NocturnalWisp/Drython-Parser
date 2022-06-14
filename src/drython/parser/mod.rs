mod function_parser;
mod variable_parser;

use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use std::fs;
use std::fmt::Write;
use super::script_type::ScriptType;
use super::types;
use super::utility;

use function_parser::parse_func;
use variable_parser::parse_var;

pub struct Parser
{
    pub script_type: ScriptType,
    pub parsed_vars: LinkedHashMap<String, String>,
    pub parsed_funcs: HashMap<String, types::Func>
}

impl Parser
{
    // Main parse function.
    pub fn parse_file(file_path: &str, parsed_errors: &mut Vec<String>) -> Result<Parser, String>
    {
        let mut contents: String = match fs::read_to_string(&file_path)
        {
            Ok(string) => string,
            Err(_) => {
                let mut s = String::new();
                #[allow(unused_must_use)]
                write!(&mut s, "Error reading from file: {}", &file_path).ok();
                return Err(s);
            }
        };

        if contents.is_empty()
        {
            let mut s = String::new();
            write!(&mut s, "Contents of {} were empty.", file_path).ok();
            return Err(s);
        }

        contents = utility::insert_line_numbers(contents);

        contents = contents
            // Replace space characters with empty.
            .replace(" ", "");
        
        // First line is script type.
        let first_line_end_index = contents.find(";");

        let script_type: ScriptType;

        if let Some(size) = first_line_end_index
        {
            let first_line = &contents[2..size];
            script_type = ScriptType::from_string(first_line);

            if let ScriptType::None = script_type
            {
                let mut s = String::new();
                write!(&mut s, "Unexpected script type: {}", first_line).ok();
                return Err(s);
            }
        }
        else { return Err("Failed to determine script type.".to_string()); }

        let mut parsed_vars: LinkedHashMap<String, String> = LinkedHashMap::new();
        let mut parsed_funcs: HashMap<String, types::Func> = HashMap::new();

        // Handle finding a func.
        let mut inside_func = false;
        let mut found_func: Vec<&str> = vec![];
        let mut func_start = 0;
        // Keeps count of internal scopes so as to not end at the wrong "end"
        let mut scope_count = 0;

        let lines: Vec<&str> = contents.split(";").collect();
        for whole_line in lines
        {
            let line: &str;
            let line_number: &str;

            if let Some(string) = &whole_line.split_once(")")
            {
                line_number = string.0;
                line = string.1;
            }
            else { /*Line Empty.*/ continue; }

            if inside_func
            {
                if utility::check_for_scope(line)
                {
                    scope_count += 1;
                }

                if line.to_lowercase() == "end"
                {
                    if scope_count == 0
                    {
                        inside_func = false;

                        let mut error_list: LinkedHashMap<usize, String> = LinkedHashMap::new();
                        match parse_func(&found_func, &mut error_list)
                        {
                            Ok(result) =>
                            {
                                parsed_funcs.insert(result.name.clone(), result);
                                
                                // Loop through function expression error list.
                                for error in error_list.iter()
                                {
                                    let mut s = String::new();
                                    write!(&mut s, "{}\n[Line:{}]\n", error.1, func_start+1+error.0).ok();
                                    parsed_errors.push(s);
                                }
                            },
                            Err(error) => 
                            {
                                // Some error through parsing function.
                                let mut s = String::new();
                                write!(&mut s, "{}\n[Line:{}]\n", error.0, func_start+1+error.1).ok();
                                parsed_errors.push(s);
                            }
                        }
                        found_func.clear();
                    }
                    else
                    {
                        scope_count -= 1;
                        found_func.push(line);
                    }
                    
                }
                else {found_func.push(line);}
            }
            else
            {
                // For variable assignement.
                if utility::ordered_chars_check(line, &['='])
                {
                    let parsed_var = parse_var(line);
                    match parsed_var
                    {
                        Ok(var) => {parsed_vars.insert(var.0, var.1);},
                        Err(error) =>
                        {
                            let mut s = String::new();
                            write!(&mut s, "{}\n[Line:{}]\n", error, line_number).ok();
                            parsed_errors.push(s);
                        }
                    }
                    continue;
                }
                // For function declerations.
                else if utility::ordered_chars_check(line, &['(', ')', ':'])
                {
                    inside_func = true;
                    found_func.push(line);
                    func_start = if let Ok(int) = line_number.parse::<usize>() {int}
                        else {panic!("Line numbers were wrong when parsing file.");};
                    continue;
                }
            }
        }

        println!("{:#?}", contents);

        Ok(Parser
        {
            script_type: script_type,
            parsed_vars: parsed_vars,
            parsed_funcs: parsed_funcs
        })
    }
}