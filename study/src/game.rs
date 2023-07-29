use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use console::{style, Term};

fn do_stuff() -> io::Result<()> {
    let term = Term::stdout();
    term.set_title("Counting...");
    term.write_line("Going to do some counting now")?;
    term.hide_cursor()?;
    for x in 0..10 {
        if x != 0 {
            term.move_cursor_up(1)?;
        }
        term.write_line(&format!("Counting {}/10", style(x + 1).cyan()))?;
        thread::sleep(Duration::from_millis(400));
    }
    term.show_cursor()?;
    term.clear_last_lines(1)?;
    term.write_line("Done counting!")?;
    writeln!(&term, "Hello World!")?;

    loop {
        write!(&term, "chat: ")?;
        // term.show_cursor()?;
        // let res = term.read_line_initial_text("default")?;
        let res = term.read_line()?;
        // term.hide_cursor()?;
        // term.move_cursor_up(1)?;
        // term.clear_line()?;
        term.clear_last_lines(1)?;
        writeln!(&term, "{}", res)?;
    }
    
    // writeln!(&term, "\n{}", res)?;
    // writeln!(&term, "{}", res)?;
    term.move_cursor_up(1)?;
    term.clear_line()?;

    Ok(())
}



pub fn run_game() {
    println!("run game");

    do_stuff().unwrap();
}

