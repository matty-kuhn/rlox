pub(crate) struct NeedsToken;
pub(crate) struct HasBang;
pub(crate) struct Finished;

/// Supports source files up to 65K lines, total of usize chars
pub(crate) struct ScannerCtx<State> {
    pub(crate) curr_line: u16,
    pub(crate) curr_col: usize,
    pub(crate) cursor: usize,
    pub(crate) errors: Vec<anyhow::Error>,
    _state: State,
}

impl ScannerCtx<NeedsToken> {
    pub(crate) fn new() -> Self {
        Self {
            curr_line: 0,
            curr_col: 0,
            cursor: 0,
            errors: vec![],
            _state: NeedsToken,
        }
    }

    pub(crate) fn finish(self) -> ScannerCtx<Finished> {
        ScannerCtx {
            curr_line: self.curr_line,
            curr_col: self.curr_col,
            cursor: self.cursor,
            errors: self.errors,
            _state: Finished,
        }
    }
}
