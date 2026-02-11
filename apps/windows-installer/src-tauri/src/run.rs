//! Run external commands in a Windows-compatible way.
//!
//! On Windows, npm and openclaw from npm install -g are typically .cmd scripts.
//! Rust's std::process::Command uses CreateProcess, which does not run .cmd/.bat
//! directly, so we run these via `cmd /c program args...` so they resolve in PATH.

use std::process::Command;

/// Programs that on Windows are usually .cmd/.bat and must be run via cmd /c.
const CMD_SCRIPT_PROGRAMS: &[&str] = &["npm", "openclaw", "npx"];

fn is_cmd_script_program(program: &str) -> bool {
    let base = program.split(['/', '\\']).last().unwrap_or(program);
    CMD_SCRIPT_PROGRAMS
        .iter()
        .any(|&p| p.eq_ignore_ascii_case(base))
}

/// Build a command that will run `program` with `args`. On Windows, if `program` is npm or
/// openclaw (or npx), runs via `cmd /c program args...` so .cmd wrappers work.
pub fn command(program: &str, args: &[&str]) -> std::process::Command {
    #[cfg(windows)]
    {
        if is_cmd_script_program(program) {
            let mut c = Command::new("cmd");
            c.args(["/C", program]);
            c.args(args);
            return c;
        }
    }
    let mut c = Command::new(program);
    c.args(args);
    c
}

/// Run `program` with `args` and return the output. Same Windows behavior as `command`.
pub fn output(program: &str, args: &[&str]) -> std::io::Result<std::process::Output> {
    command(program, args).output()
}
