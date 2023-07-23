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

// use dialoguer::Editor;
// use dialoguer::{theme::ColorfulTheme, Input};

// use std::error::Error;
// use std::net::IpAddr;

// use console::Style;
// use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

// #[derive(Debug)]
// #[allow(dead_code)]
// struct Config {
//     interface: IpAddr,
//     hostname: String,
//     use_acme: bool,
//     private_key: Option<String>,
//     cert: Option<String>,
// }

// fn init_config() -> Result<Option<Config>, Box<dyn Error>> {
//     let theme = ColorfulTheme {
//         values_style: Style::new().yellow().dim(),
//         ..ColorfulTheme::default()
//     };
//     println!("Welcome to the setup wizard");

//     if !Confirm::with_theme(&theme)
//         .with_prompt("Do you want to continue?")
//         .interact()?
//     {
//         return Ok(None);
//     }

//     let interface = Input::with_theme(&theme)
//         .with_prompt("Interface")
//         .default("127.0.0.1".parse().unwrap())
//         .interact()?;

//     let hostname = Input::with_theme(&theme)
//         .with_prompt("Hostname")
//         .interact()?;

//     let tls = Select::with_theme(&theme)
//         .with_prompt("Configure TLS")
//         .default(0)
//         .item("automatic with ACME")
//         .item("manual")
//         .item("no")
//         .interact()?;

//     let (private_key, cert, use_acme) = match tls {
//         0 => (Some("acme.pkey".into()), Some("acme.cert".into()), true),
//         1 => (
//             Some(
//                 Input::with_theme(&theme)
//                     .with_prompt("  Path to private key")
//                     .interact()?,
//             ),
//             Some(
//                 Input::with_theme(&theme)
//                     .with_prompt("  Path to certificate")
//                     .interact()?,
//             ),
//             false,
//         ),
//         _ => (None, None, false),
//     };

//     Ok(Some(Config {
//         hostname,
//         interface,
//         private_key,
//         cert,
//         use_acme,
//     }))
// }


pub fn run_game() {
    println!("run game");

    do_stuff().unwrap();

    // match init_config() {
    //     Ok(None) => println!("Aborted."),
    //     Ok(Some(config)) => println!("{:#?}", config),
    //     Err(err) => println!("error: {}", err),
    // }

    // if let Some(rv) = Editor::new().edit("Enter a commit message").unwrap() {
    //     println!("Your message:");
    //     println!("{}", rv);
    // } else {
    //     println!("Abort!");
    // }

    // let input: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Your name")
    //     .interact_text()
    //     .unwrap();

    // println!("Hello {}!", input);

    // let mail: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Your email")
    //     .validate_with({
    //         let mut force = None;
    //         move |input: &String| -> Result<(), &str> {
    //             if input.contains('@') || force.as_ref().map_or(false, |old| old == input) {
    //                 Ok(())
    //             } else {
    //                 force = Some(input.clone());
    //                 Err("This is not a mail address; type the same value again to force use")
    //             }
    //         }
    //     })
    //     .interact_text()
    //     .unwrap();

    // println!("Email: {}", mail);

    // let mail: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Your planet")
    //     .default("Earth".to_string())
    //     .interact_text()
    //     .unwrap();

    // println!("Planet: {}", mail);

    // let mail: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Your galaxy")
    //     .with_initial_text("Milky Way".to_string())
    //     .interact_text()
    //     .unwrap();

    // println!("Galaxy: {}", mail);
}

