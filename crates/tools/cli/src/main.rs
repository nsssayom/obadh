// crates/tools/cli/src/main.rs

use obadh_engine::processor::Processor;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Obadh Bengali Input Method - Test Console");
    println!("Type English characters for Bengali output (press Ctrl+C to exit)");

    let mut processor = Processor::new();
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Process the entire input string
        let output = processor.process_input(input);
        println!("{}", output);
    }
}
