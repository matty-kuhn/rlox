use rlox::interpreter::{Interpreter, InterpreterArgs, Parser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = InterpreterArgs::parse();

    Ok(Interpreter::new(args).run()?)
}
