use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  let max:u32 = 100;
  let min:u32 = 1;
  println!("Guess the number between {min} and {max}!");
  let num:u32 = rand::thread_rng().gen_range(min..=max);
  let mut attempts:u32 = 0;
  loop {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
      Ok(n) => n,
      Err(_) => {
        println!("Please enter a number!");
        continue
      }
    };
    if (min..=max).contains(&guess) {
      println!("Got {guess}");
    } else {
      println!("Please enter a number between {min} and {max}!");
      continue
    }
    attempts = attempts + 1;

    match guess.cmp(&num) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {println!("Correct!"); 
                          break}
    }
  }
  println!("Took {attempts} attempt(s)")
}
