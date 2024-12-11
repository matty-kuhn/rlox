use std::marker::PhantomData;

pub(crate) trait CtxState {}
pub(crate) trait Parsing: CtxState {}
pub(crate) struct Started;
impl CtxState for Started {}
impl Parsing for Started {}
pub(crate) struct HasBang;
impl CtxState for HasBang {}

/// Supports source files up to 65K lines, total of usize chars
pub(crate) struct ScannerCtx<State>
where
    State: CtxState,
{
    pub(crate) curr_line: u16,
    pub(crate) curr_col: usize,
    pub(crate) cursor: usize,
    pub(crate) errors: Vec<anyhow::Error>,
    _state: PhantomData<State>,
}

impl<State> ScannerCtx<State>
where
    State: CtxState,
{
    pub(crate) fn new() -> Self {
        Self {
            curr_line: 0,
            curr_col: 0,
            cursor: 0,
            errors: vec![],
            _state: std::marker::PhantomData,
        }
    }
}
