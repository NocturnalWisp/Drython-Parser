pub mod drython;

use drython::types::Parser;
use linked_hash_map::LinkedHashMap;

use crate::drython::types::Runner;

fn main()
{
    let mut parse_warnings: LinkedHashMap<usize, String> = LinkedHashMap::new();

    match Parser::parse_file("data/test.dry", &mut parse_warnings)
    {
        Result::Ok(result) =>
        {
            println!("{:#?}", result);

            let mut runner = Runner::new(result);

            runner.run_setup();

            runner.call_function("callme", vec![]);
        },
        Result::Err(error) =>
        {
            println!("Failed to parse file due to:\n\n{}\n", error);
        }
    }
}
