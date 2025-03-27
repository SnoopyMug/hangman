use std::io::{self};

fn print_head(m: usize) -> String
{
    let s: String = String::from("O");

    if m >= 1{
        return s;
    }
    else{
        return String::from("");
    }
}

fn random_word() -> Vec<char>
{
    let word: String = String::from("hello");
    return word.chars().collect();
}

fn build_guess(n: usize) -> Vec<char>
{
    let mut guess: Vec<char> = Vec::new();
    let mut count: usize = 0;
    while count < n {
        guess.push('_');
        count += 1;
    }
    
    return guess;
}

fn check_guess(input: &str, guesses:&mut Vec<char>, word: &Vec<char>)
{
    for n in 0..word.len()
    {
        if input.chars().next() == Some(word[n])
        {
            guesses[n] = word[n];
        }
    }    

    let word_string: String = word.into_iter().collect();
    let guesses_string: String = guesses.iter().collect();

    if input == word_string || guesses_string == word_string
    {
        *guesses = word.clone();
        println!("You win!")
    }

}

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let mut input: String = String::new();
    let word: Vec<char> = random_word();
    let mut guess: Vec<char> = build_guess(word.len());
    let mut mistakes: Vec<String> = Vec::new();
    loop{
        println!("---------");
        println!("|       |");
        println!("|       {}", print_head(mistakes.len()));
        println!("|");
        println!("|");
        println!("|");
        println!("|");
        println!("|");
        println!("---------");  
        let guess_string: String = guess.clone().into_iter().collect();
        println!("\n{}", guess_string);
        println!("Type a letter or word");
        
        io::stdin().read_line(&mut input)?;
        match input.trim() {
            "exit" => break,
            _ => {// Any input
                println!("You typed: {}", input.trim());
                check_guess(input.trim(), &mut guess, &word);
            },
        }
        input.clear();
    }
    
    Ok(()) // Explicitly needed because of Result
}
