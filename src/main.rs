use std::io::{self};

fn random_answer() -> Vec<char>
{
    let answer: String = String::from("hello");
    return answer.chars().collect();
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

fn print_mistakes(mistakes: &Vec<String>) -> String
{
    let mut s: String = String::from("");
    for n in mistakes
    {
        s = s + &n + " ";
    }

    return s;

}

fn check_guess(input: &str, guesses:&mut Vec<char>, answer: &Vec<char>, mistakes:&mut Vec<String>)
{
    let mut mistake: bool = true;
    for n in 0..answer.len()
    { //Iterate over each character of the answer
        if input.chars().next() == Some(answer[n])
        {
            mistake = false;
            guesses[n] = answer[n];
        }

    }    



    if mistake == true
    {
        mistakes.push(input.to_string());
    }

}

enum GameState
{
    InProgress,
    End,
}

fn main() -> io::Result<()> {
    let mut input: String = String::new();
    let mut answer: Vec<char> = random_answer();
    let mut guess: Vec<char> = build_guess(answer.len());
    let mut mistakes: Vec<String> = Vec::new();
    let mut cur_state: GameState = GameState::InProgress;
    loop{

        let ml: usize = mistakes.len();
        let answer_string: String = answer.iter().collect();
        let guesses_string: String = guess.iter().collect();

        if (input == answer_string || guesses_string == answer_string) ||
           (ml >= 6)
        {
            cur_state = GameState::End;
        }
        println!("---------");
        println!("|       |");
        println!("|       {}", if ml >= 1 {"0"} else {""}); // We want to check a range for the size
        println!("|      {}", if ml == 2 {" |"} 
                              else if ml == 3 {"/|"}
                              else if ml >= 4 {"/|\\"}
                              else {""}
                            );
        println!("|      {}", if ml == 5 {"/"}
                              else if ml >= 6 {" /\\"}
                              else {""}
                            );
        println!("|");
        println!("|");
        println!("|");
        println!("---------");  
        let guess_string: String = guess.clone().into_iter().collect();
        println!("\n{}", guess_string);
        println!("MISTAKES: {}", print_mistakes(&mistakes));
        println!("{}", if guess_string == answer_string {"YOU WIN!"} 
                        else if ml >= 6 {"YOU LOSE!"}
                        else {"Please type a letter"}
                        );
        

        match cur_state {
            GameState::InProgress => { 
                io::stdin().read_line(&mut input)?;
                println!("You typed: {}", input.trim());
                check_guess(input.trim(), &mut guess, &answer, &mut mistakes);
            },
            GameState::End => {
                println!("Type 'exit' to exit or any key to restart");
                io::stdin().read_line(&mut input)?;
                answer.clear();
                answer = random_answer();
                guess.clear();
                guess  = build_guess(answer.len());
                mistakes.clear();
                cur_state = GameState::InProgress;
                if input.trim() == "exit"
                {
                    break; // Exit the loop to make Ok(()) reachable
                }
            }  
        }
        
        input.clear();
    }
    
    Ok(()) // Explicitly needed because of Result
}
