use crate::setup::Start;

pub(crate) struct ConsoleUserRequestHandler;

#[async_trait::async_trait]
impl Start for ConsoleUserRequestHandler {
    type E = std::io::Error;

    async fn start(self) -> Result<(), Self::E> {
        println!("Console client");

        loop {
            print!("> ");

            let mut command = String::new();
            std::io::stdin().read_line(&mut command)?;

            let command: Vec<&str> = command.split(" ").collect();
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
