use std::io::stdout;
use crossterm::ExecutableCommand;
use crossterm::cursor::{MoveLeft, MoveUp};

/// Common trait for all printable components that needs a reset during refresh.
pub trait RefreshableComponent {

    /// Resets the cursor to the top left corner.
    fn reset_cursor(&self) {
        stdout()
            .execute(MoveLeft(self.after_print_cols_offset())).unwrap()
            .execute(MoveUp(self.after_print_rows_offset())).unwrap();
    }

    /// The offset after each print in cols the cursor needs to be
    /// reset to the left.
    fn after_print_cols_offset(&self) -> u16;

    /// The offset after each print in cols the cursor needs to be
    /// reset to the top.
    fn after_print_rows_offset(&self) -> u16;

    /// Pretty-Prints the component using ANSI sequences for colors etc.
    fn print(&self);
}
