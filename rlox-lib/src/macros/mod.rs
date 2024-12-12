// TODO: other macro for err context reporting?

#[macro_export]
/// Used to generate a helpful error message, used in conjuction with [`anyhow::anyhow`]
///
/// Example:
/// ```
/// # use rlox_lib::err_msg;
/// // will output: "[line: 42] Error: missing comma!"
/// let error = err_msg!(42, "missing comma!");
/// assert_eq!(error, "[line: 42] Error: missing comma!");
/// ```
macro_rules! err_msg {
    ($line_nr: expr, $message: expr) => {
        format!("[line: {}] Error: {}", $line_nr, $message)
    };
    ($line_nr: expr, $message: expr, $col_nr: expr) => {
        format!(
            "[line: {} column: {}] Error: {}",
            $line_nr, $col_nr, $message
        )
    };
    ($line_nr: expr, $message: expr, $ctx: block) => {
        format!("[line: {}] Error: {}\n\t\t{}", $line_nr, $message, $ctx)
    };
}

#[cfg(test)]
mod test {

    #[test]
    fn test_err_msg_gen() {
        let error = err_msg!(37, "missing comma!");
        assert_eq!(error, "[line: 37] Error: missing comma!");

        struct Dummy {
            val: usize,
        }
        let val = 42;

        let error = err_msg!(val, "missing comma!");
        assert_eq!(error, "[line: 42] Error: missing comma!");

        let dum = Dummy { val };

        let error = err_msg!(dum.val, format!("missing {}", "comma!"));
        assert_eq!(error, "[line: 42] Error: missing comma!");
    }
}
