use anyhow::{Context, Result};
use rustyline::DefaultEditor;

pub(crate) struct Repl {
    line_reader: DefaultEditor,
    ctx: ReplCtx,
}

impl Repl {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            line_reader: DefaultEditor::new()?,
            ctx: ReplCtx::default(),
        })
    }

    pub(crate) fn run(mut self) -> Result<()> {
        loop {
            let rl = self
                .line_reader
                .readline("lox >>>")
                .context("could not read line from terminal")?;
            if rl == "exit" || rl == "quit" {
                return Ok(());
            }
            if let Err(e) = super::run(&rl, Some(&mut self.ctx)) {
                println!("{e}");
            }
        }
    }
}

#[derive(Default)]
pub struct ReplCtx {}
