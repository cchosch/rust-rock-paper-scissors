use rand::Rng;
use std::collections::HashMap;
use std::io::{stdin,stdout,Write};

fn main() {
    let mut rando = rand::thread_rng();
    let mut winners = HashMap::new();
    let choices = ["Rock", "Scissors", "Paper"];
    winners.insert("Rock", "Scissors");
    winners.insert("Scissors", "Paper");
    winners.insert("Paper", "Rock");
    loop{
        let mut line = String::new();
        let computer_choice = choices[rando.gen_range(0..3)];

        stdout().flush().expect("error flushing");
        stdin().read_line(&mut line).expect("what are you doing");
        line = String::from(line.trim());

        if winners.get(&line).expect("Not a value") == computer_choice {

        }
    }
}