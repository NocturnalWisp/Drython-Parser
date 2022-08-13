pub mod drython;

use drython::types::Parser;
use linked_hash_map::LinkedHashMap;

use crate::drython::types::Runner;
use crate::drython::types::error::ErrorManager;

fn main()
{
    let mut error_manager = ErrorManager::new();
    let mut parse_warnings: LinkedHashMap<usize, String> = LinkedHashMap::new();

    match Parser::parse_file("data/test.dry", &mut error_manager)
    {
        Result::Ok(result) =>
        {
            let mut runner = Runner::new(result);

            runner.run_setup(&mut error_manager);

            runner.call_function("callme", vec![]);

            if parse_warnings.len() > 0
            {
                println!("{} contained the following issues:", "data/test.dry");
                println!("{:?}", error_manager.errors.iter().map(|e| e.display()).collect::<Vec<String>>());
            }
        },
        Result::Err(error) =>
        {
            println!("Failed to parse file due to:\n\n{}\n", error);
        }
    }
}
