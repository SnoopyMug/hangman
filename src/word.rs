use std::fs::read_to_string;
use rand::seq::IndexedRandom;
use rand::rng;

pub fn read_lines(filename: &str) -> Vec<String>
{
    return read_to_string(filename)
        .unwrap() // panic on possible file read errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() //gather into a vector
}

pub fn random_answer() -> Vec<char>
{
    let dictionary_file: &str = "assets/words.txt";
    let dictionary: Vec<String> = read_lines(dictionary_file);
    
    let mut rng = rng(); // random number

    if let Some(random_value) = dictionary.choose(&mut rng) 
    {
        return random_value.chars().collect();
    }
    else 
    {
        println!("Word is empty!");
        return Vec::new();
    }
}

pub fn build_guess(n: usize) -> Vec<char>
{
    let mut guess: Vec<char> = Vec::new();
    let mut count: usize = 0;
    while count < n {
        guess.push('_');
        count += 1;
    }
    
    return guess;
}

pub fn print_mistakes(mistakes: &Vec<String>) -> String
{
    let mut s: String = String::from("");
    for n in mistakes
    {
        s = s + &n + " ";
    }

    return s;

}

pub fn check_guess(input: &str, guesses:&mut Vec<char>, answer: &Vec<char>, mistakes:&mut Vec<String>)
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