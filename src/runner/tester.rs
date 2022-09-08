use crate::types::Runner;
use crate::types::error::*;

pub fn test_runner(runner: &mut Runner, _error_manager: &mut ErrorManager)
{
    // runner.run_setup(error_manager);



    // Remove all saved stuff.
    runner.external_functions.clear();
    runner.vars.clear();
}
