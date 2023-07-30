use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// console crates.
use console::{style, Term};

fn main() {
    do_stuff().unwrap();
}

fn do_stuff() -> io::Result<()> {
    let term = Term::stdout();
    term.clear_last_lines(1)?;
    term.write_line("Done counting!")?;
    writeln!(&term, "Hello World!")?;

    loop {
        write!(&term, "chat: ")?;
        
        let res = term.read_line()?;
        term.clear_last_lines(1)?;
        writeln!(&term, "{}", res)?;
    }

    // term.move_cursor_up(1)?;
    // term.clear_line()?;

    Ok(())
}
