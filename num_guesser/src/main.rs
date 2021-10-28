use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hey! Welcome to the number guessing game!");
    let answer: u32 = rand::thread_rng().gen_range(1..101);
    let mut score =101;
    loop{
    score=score-1;
    println!("Enter your guess:");
    let mut guess= String::new();
    io::stdin()
               .read_line(&mut guess)
               .expect("Invlid Input");
    let guess: u32=guess.trim().parse().expect("Enter a number");
    match answer.cmp(&guess){
    Ordering::Less => println!("go lower!"),
    Ordering::Greater => println!("go higher!"),
    Ordering::Equal=> {
                       println!("Lessgoooo! correct answer");
                       println!("Your Score : { }",score);
                       break;
                       }
    }
    }             
}


