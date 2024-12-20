use anyhow::{bail, Result};
use std::{
    fs::{self},
    path::Path,
};

pub(crate) struct FileRunner<'path> {
    file: &'path Path,
}

impl<'path> FileRunner<'path> {
    pub(crate) fn new(file: &'path Path) -> Self {
        Self { file }
    }

    pub(crate) fn run(self) -> Result<()> {
        match self.file.extension() {
            Some(ext) => {
                if ext != "lox" {
                    bail!("only .lox files may be run")
                }
            }
            None => bail!("only .lox files may be run"),
        }

        let file_contents = fs::read_to_string(self.file)?;

        super::run(&file_contents, None)
    }
}
