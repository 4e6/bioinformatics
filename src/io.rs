use std::io;

#[cfg(windows)] pub const NL: &'static str = "\r\n";
#[cfg(not(windows))] pub const NL: &'static str = "\n";

static FAILED_TO_READ_LINE: &'static str = "Failed to read line";

/// read line from stdin,
/// drops trailing newline
pub fn read_line(s: &mut String) {
    io::stdin().read_line(s)
        .expect(FAILED_TO_READ_LINE);
    s.pop();
}
