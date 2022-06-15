pub mod drython;

use std::io::stdin;
use drython::parser::Parser;
use linked_hash_map::LinkedHashMap;

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

        let mut parse_warnings: LinkedHashMap<usize, String> = LinkedHashMap::new();

        match Parser::parse_file("data/test.dry", &mut parse_warnings)
        {
            Result::Ok(result) =>
            {
                println!("{:#?}", result);
            },
            Result::Err(error) =>
            {
                println!("Failed to parse file due to:\n\n{}\n", error);
            }
        }
    }
}
