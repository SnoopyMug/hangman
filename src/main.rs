use std::io;

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let mut input = String::new();
    loop{
        println!("Enter a letter");
        io::stdin().read_line(&mut input)?;
        match input.trim() {
            "exit" => break,
            _ => {// Any input
                println!("You typed: {}", input.trim());
            },
        }
        input.clear();
    }
    Ok(()) // Explicitly needed because of Result
}
