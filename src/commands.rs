use std::io;

pub struct Command {
    name: String,
    handler: fn()
}

impl Command {
    pub fn new(name: &str, handler: fn()) -> Command {
        Command {
            name: String::from(name),
            handler: handler
        }
    }

    pub fn handle(&self) {
        (self.handler)();
    }
}

impl Drop for Command {
    fn drop(&mut self) {
        println!("{} command 소멸자.", self.name);
    }
}

pub struct CommandHandler {
    pub name: String,
    commands: Vec<Command>
}

impl CommandHandler {
    pub fn new<T>(name: &str, iter: T) -> CommandHandler
    where T: Iterator<Item = Command> {
        CommandHandler {
            name: String::from(name),
            commands: Vec::from_iter(iter)
        }
    }

    pub fn handle(&self) {
        let result = self.select();
    
        match result {
            Err(error) => {
                println!("{error}");
                return;
            },
            Ok(selected_command) => {
                match selected_command {
                    None => {
                        println!("{} is empty.", self.name);
                        return;
                    },
                    Some(selected_command) => {
                        selected_command.handle();
                    }
                }
            }
        }
    } 

    fn select(&self) -> Result<Option<&Command>, String> {
        println!("{}", self.name);

        if self.commands.is_empty() {
            return Ok(None);
        }

        for i in 0..self.commands.len() {
            let no = i+1;
            
            println!("{}) {}", no, self.commands[i].name);
        }

        let mut input_string = String::new();

        io::stdin().read_line(&mut input_string).expect("failed to readline.");
        
        let selected_no = input_string.trim().parse::<usize>().unwrap();

        if self.commands.len() < selected_no {
            return Err(format!("selected no ({selected_no}) is invalid."));
        }

        let command = &self.commands[selected_no - 1];
    
        return Ok(Some(command));
    }
}

