#[path="./../types.rs"]
pub mod types;
#[path="./../utility.rs"]
pub mod utility;

#[path="./../parser/mod.rs"]
mod parser;

mod var;
mod func;

use parser::Parser;

pub fn trial_run(parser: &Parser)
{
    
}