use crate::setup::Start;

pub(crate) struct ConsoleUserRequestHandler;

#[async_trait::async_trait]
impl Start for ConsoleUserRequestHandler {
    type E = std::io::Error;

    async fn start(self) -> Result<(), Self::E> {
        println!("Console client");

        'clinet_main_loop: loop {
            print!("> ");

            let mut command = String::new();
            std::io::stdin().read_line(&mut command)?;

            let command: Vec<&str> = command.split(" ").collect();
            let command = command.as_slice();

            match command {
                &["echo", text] => { println!("{}", text); },
                &["quit", _] => { break 'clinet_main_loop; }
                _ => { println!("[!] Unrecognized command."); }
            }
        }

        Ok(())
    }
}
