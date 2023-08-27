use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

// console crates.
use console::{style, Term};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // do_stuff().unwrap();

    println!("1 {:?}", thread::current().id());

    let rt = Runtime::new()?;

    rt.block_on(async {
        println!("2 {:?}", thread::current().id());

        for i in 0..100 {
            let result =tokio::spawn(async {
                println!("3 {:?}", thread::current().id());
    
            }); //.await;
        }

        println!("4 {:?}", thread::current().id());
    });

    Ok(())
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
