use std::io::{self, Write};

use crate::setup::Start;

pub(crate) struct ConsoleUserRequestHandler;

#[async_trait::async_trait]
impl Start for ConsoleUserRequestHandler {
    type E = io::Error;

    async fn start(self) -> Result<(), Self::E> {
        println!("Console client");

        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut command = String::new();
            io::stdin().read_line(&mut command)?;

            let command: Vec<&str> = command.strip_suffix("\n").unwrap().split(" ").collect();
            let command = command.as_slice();

            if command[0] == "quit" || command[0] == "exit" {
                break;
            }

            self.interprete(command);
        }

        Ok(())
    }
}

impl ConsoleUserRequestHandler {
    fn interprete(&self, command: &[&str]) {
        match command {
            &["echo", text] => { println!("{}", text); },
            _ => { println!("[!] Unrecognized command."); }
        }
    }
}
