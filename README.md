# Drython-Parser
A parser for the drython scripting language. Programmed in Rust. Currently only the parser is implemented, but I'm working on the runner!

## Running The Parser
1. Install rust to your computer https://www.rust-lang.org/tools/install
2. Clone this repo.
3. Open the repo folder in cmd. Run "cargo run"

## Design Considerations
Unfortunately, since this is my first language, there are missing features, bugs, or inneficient algorithms. This is an inexaustive list of the strengths and weaknesses of Drython.
- Error catching for parsing is sub-par. The way the language is parsed results in difficult to determine error placement. A second step called "tester" could be included in the source code, but may not be as effective with lack of token specifying.
- Drython parser uses both AST trees and Reverse Polish Notation for operation handling. This could be both good and bad because there is no tree traversal to find the more important operation, but it uses more structures than the polish notation would. Give and take.
- With drython not detecting types until they are used, (arguments, assigments) the error handling doesn't occur until it's run. (When the game is played in editor.) But it also means the language is very dynamic and should handle parsing behind the scenes. (Variables can be overriden like in java - member variables.)
