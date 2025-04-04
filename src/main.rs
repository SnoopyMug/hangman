use std::io::{self};
pub mod word;

enum GameState
{
    InProgress,
    End,
}

fn main() -> io::Result<()> {
    let mut input: String = String::new();
    let mut answer: Vec<char> = word::random_answer();
    let mut guess: Vec<char> = word::build_guess(answer.len());
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
        println!("\nYOUR ANSWER: {}", guess_string);
        println!("MISTAKES: {}", word::print_mistakes(&mistakes));
        println!("{}", if guess_string == answer_string {"YOU WIN!".to_string()} 
                        else if ml >= 6 {format!("YOU LOSE! The answer was {}", answer_string)}
                        else {"Please type a letter".to_string()}
                        );
        

        match cur_state {
            GameState::InProgress => { 
                io::stdin().read_line(&mut input)?;
                println!("You typed: {}", input.trim());
                word::check_guess(input.trim(), &mut guess, &answer, &mut mistakes);
            },
            GameState::End => {
                println!("Type 'exit' to exit or any key to restart");
                io::stdin().read_line(&mut input)?;
                if input.trim() == "exit"
                {
                    break; // Exit the loop to make Ok(()) reachable
                }
                else
                {
                    answer.clear();
                    answer = word::random_answer();
                    guess.clear();
                    guess  = word::build_guess(answer.len());
                    mistakes.clear();
                    cur_state = GameState::InProgress;
                }
            }  
        }
        
        input.clear();
    }
    
    Ok(()) // Explicitly needed because of Result
}
