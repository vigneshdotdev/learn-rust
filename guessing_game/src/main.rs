use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Hey buddy, You can guess if you can. Lets challange :)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess =  Guess::new(match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        });
        println!("You guessed {:#?}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Wallah, You win!");
                break;
            }
        }
    }
}


#[derive(Debug)]
struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess{
        if value < 0 || value > 100 {
            panic!("Guess should be lessthan 100 and greater than 0");
        }
        Guess {value}
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
