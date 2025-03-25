use crossterm::{
    cursor, event::{self, KeyCode}, execute, terminal,
    ExecutableCommand
};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let options = vec!["option 1", "option 2", "option 3"];
    let mut selected_index = 0;

    let mut stdout = io::stdout();
    terminal::enable_raw_mode().unwrap();

    loop{
        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout.execute(cursor::MoveTo(0,0)).unwrap();

        for (i, option) in options.iter().enumerate() {
            if i == selected_index {
                println!("> {} <", option); // selected options
            }
            else {
                println!("{}", option); // non selected options
            }
        }

    stdout.flush().unwrap();

    if event::poll(std::time::Duration::from_millis(100)).unwrap(){ // check every 100 seconds
        if let event::Event::Key(key_event) = event::read().unwrap() {
            match key_event.code{

                KeyCode::Up => {
                    if selected_index > 0{
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < options.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Esc => break,
                KeyCode::Enter => {
                    println!("\n{} selected", options[selected_index]);
                    break;
                }
                _ => {}
                }
            }
        }
    }

    terminal::disable_raw_mode().unwrap();
    /*let mut input = String::new();
    loop{
        println!("Select an option");
        
        println!("e - Exit")
        io::stdin().read_line(&mut input)?;
        match input.trim() {
            "exit" => break,
            _ => {// Any input
                println!("You typed: {}", input.trim());
            },
        }
        input.clear();
    }
    */
    Ok(()) // Explicitly needed because of Result
}
