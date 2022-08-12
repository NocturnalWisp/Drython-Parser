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
            println!("{:#?}", result.global_expressions);
            let mut runner = Runner::new(result);

            runner.run_setup();

            runner.call_function("callme", vec![]);

            if parse_warnings.len() > 0
            {
                println!("{} contained the following issues:", "data/test.dry");
                println!("{:?}", parse_warnings);
            }
        },
        Result::Err(error) =>
        {
            println!("Failed to parse file due to:\n\n{}\n", error);
        }
    }
}
