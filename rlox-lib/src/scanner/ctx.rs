pub(crate) struct NeedsToken;
pub(crate) struct Finished;

/// Supports source files up to 65K lines, total of usize chars
pub(crate) struct ScannerCtx {
    pub(crate) curr_line: u16,
    pub(crate) curr_col: usize,
    pub(crate) errors: Vec<anyhow::Error>,
    pub(crate) cursor: usize,
}

impl ScannerCtx {
    pub(crate) fn new() -> Self {
        Self {
            curr_line: 0,
            curr_col: 0,
            cursor: 0,
            errors: vec![],
        }
    }

    pub(crate) fn newline(&mut self) {
        self.curr_line += 1;
        self.curr_col = 0;
    }

    pub(crate) fn advance(&mut self) {
        self.cursor += 1;
        self.curr_col += 1;
    }
}
