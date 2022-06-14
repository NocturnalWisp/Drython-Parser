pub mod drython;

use std::io::stdin;
use drython::parser::Parser;

fn main()
{
    loop
    {
        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        
        // Stop looping if any other input is given than hitting return.
        if !input.trim().is_empty() { break; }

        let mut parsed_errors = Vec::new();

        match Parser::parse_file("data/test.dry", &mut parsed_errors)
        {
            Result::Ok(result) =>
            {
                println!("{:?}", result.script_type);
                println!("{:?}", result.parsed_vars);
                println!("{:#?}", result.parsed_funcs);
            },
            Result::Err(error) =>
            {
                println!("Failed to parse file due to:\n\n{}\n", error);
            }
        }
    }
}
