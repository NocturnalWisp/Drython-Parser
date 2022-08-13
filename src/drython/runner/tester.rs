use super::super::types::Runner;
use super::super::types::error::*;

pub fn test_runner(runner: &mut Runner, error_manager: &mut ErrorManager)
{
    runner.run_setup(error_manager);



    // Remove all saved stuff.
    runner.internal_functions.clear();
    runner.external_functions.clear();
    runner.vars.clear();
}
