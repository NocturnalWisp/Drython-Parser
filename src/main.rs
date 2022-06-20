pub mod drython;

use std::io::stdin;
use drython::types::Parser;
use linked_hash_map::LinkedHashMap;

fn main()
{
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
