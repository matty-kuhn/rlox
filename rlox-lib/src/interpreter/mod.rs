mod file_runner;
mod repl;

pub use clap::Parser;

use anyhow::{bail, Context, Result};
use file_runner::FileRunner;
use repl::{Repl, ReplCtx};
use std::path::PathBuf;

use crate::scanner::{ctx::Started, Scanner};

#[derive(Parser, Debug)]
pub struct InterpreterArgs {
    /// The name of the file to run
    #[arg(index = 1)]
    pub file: Option<PathBuf>,
}

pub struct Interpreter {
    args: InterpreterArgs,
}

impl Interpreter {
    pub fn new(args: InterpreterArgs) -> Self {
        Self { args }
    }

    pub fn run(self) -> Result<()> {
        if let Some(file) = self.args.file {
            FileRunner::new(&file).run()
        } else {
            Repl::new()?.run()
        }
    }
}

pub(crate) fn run(code: &str, ctx: Option<&mut ReplCtx>) -> Result<()> {
    let mut scanner: Scanner<Started> = Scanner::new(code).into_iter();

    while let Some(token) = scanner.next() {
        let token = token.context("Lexing Error")?;
        println!("{token}");
    }

    if scanner.has_errors() {
        for error in scanner.errors() {
            eprintln!("error");
        }
        bail!("Errors during lexing");
    }
    Ok(())
}
