extern crate rand;

mod color;
mod peg;

use color::Color;
use rand::Rng;
use std::io;
use io::Write;
use peg::Peg;


macro_rules! flush {
    () => (io::stdout().flush().expect("Flush failed..."));
}

fn main() {
    let num_pegs = get_num_pegs();
    let answer = get_answer(num_pegs);
    let (solved, guess_counter) = play(&answer, num_pegs);
    if solved {
        let try_tries = match 12 - guess_counter {
            1 => "try",
            _ => "tries",
        };
        print!("Congratulations! You guessed the code with {} {} remaining! The code was ",
               12 - guess_counter, try_tries);
    } else {
        print!("Sorry, you're out of guesses. The code was ");
    }
    for i in 0..answer.len() - 1 {
        print!("{}, ", answer[i])
    }
    print!("{}.", answer[answer.len() - 1]);
}

fn get_num_pegs() -> u32 {
    let mut pegs: u32 = 1;
    while pegs < 2 {
        print!("Enter the number of pegs in the code: ");
        flush!();
//        io::stdout().flush().expect("Flush failed...");
        let mut num_pegs = String::new();
        io::stdin().read_line(&mut num_pegs).expect("Something failed.");
        pegs = match num_pegs.trim().parse::<u32>() {
            Ok(n) => {
                if n > 1 {
                    n
                } else {
                    println!("Please enter a number greater than 1.");
                    0
                }
            }
            Err(_) => {
                println!("Please enter a number greater than 1.");
                0
            }
        };
    }
    pegs
}

fn get_answer(num_pegs: u32) -> Vec<Peg> {
    let mut pegs: Vec<Peg> = Vec::new();
    for _ in 0..num_pegs {
        pegs.push(
            Peg::new(
                Color::new(rand::thread_rng().gen_range(0, color::NUM_COLORS))
            )
        );
    }
    pegs
}

fn play(answer: &Vec<Peg>, num_pegs: u32) -> (bool, u32) {
    let mut guess_counter = 1;
    let mut solved = false;
    while guess_counter < 13 {
        let mut guess = String::new();
        let mut modified_guess = String::new();
        while guess == "" {
            print!("{}/12\tEnter your guess: ", guess_counter);
            flush!();
//            io::stdout().flush().expect("Flush failed...");
            match io::stdin().read_line(&mut guess) {
                Ok(_) => {}
                Err(err) => println!("Uh oh! {}", err),
            }
//            guess.retain(|c| { // unstable code
//                c.is_alphabetic()
//            });

            modified_guess = guess.chars().filter(|c| {
                c.is_alphabetic()
            }).collect::<String>();
            if modified_guess.chars().count() as u32 != num_pegs {
                println!("\t\tPlease enter {} letters.", &num_pegs);
                guess = String::new()
            }
        }
        let mut guess_convert = peg::convert(modified_guess);
        let result = check_answer(&mut guess_convert, &mut answer.clone());
        print!("\t\t");
        if result.len() > 0 {
            for r in result.chars() {
                print!("{} ", r)
            }
        } else {
            print!("-")
        }
        println!();
        if !result.contains("0") && result.len() as u32 == num_pegs {
            solved = true;
            break;
        } else {
            guess_counter += 1
        }
    }
    (solved, guess_counter)
}

fn check_answer(guess: &mut Vec<Peg>, answer: &mut Vec<Peg>) -> String {
    let mut result: Vec<&str> = Vec::new();
    for i in 0..guess.len() {
        if guess[i].color() == answer[i].color() && !answer[i].found() {
            result.push("1");
            answer[i].find();
            guess[i].find();
        }
    }
    guess.retain(|p| {
        !p.found()
    });

    answer.retain(|p| {
        !p.found()
    });

    for g in guess.iter() {
        if answer.contains(g) {
            let index = answer.iter().position(|p| {
                p.color() == g.color() && !p.found() && !g.found()
            }).unwrap();
            if !answer[index].found() {
                result.push("0");
                answer[index].find();
            }
        } else {
            continue;
        }
    }
    result.join("") // TODO separator needed?
}
