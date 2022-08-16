pub mod drython;

use drython::types::Parser;

use crate::drython::types::Runner;
use crate::drython::types::error::ErrorManager;

fn main()
{
    let mut error_manager = ErrorManager::new();

    match Parser::parse_file("data/test.dry", &mut error_manager)
    {
        Result::Ok(result) =>
        {
            let mut runner = Runner::new(result);

            println!("{:#?}", runner.parser.global_expressions);

            runner.run_setup(&mut error_manager);

            runner.call_function("callme", vec![], &mut error_manager);

            if error_manager.errors.len() > 0
            {
                println!("{} contained the following issues:", "data/test.dry");
                println!("{:#?}", error_manager.errors.iter().map(|e| e.display()).collect::<Vec<String>>());
            }
        },
        Result::Err(error) =>
        {
            println!("Failed to parse file due to:\n\n{}\n", error);
        }
    }
}
