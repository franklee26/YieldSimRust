use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn practise() {
    // generate random number
    let secret_number = rand::thread_rng().gen_range(1,101);
    loop {
        // make mutable string
        println!("\nPlease input your guess:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");  

        // want to compare, so I need an integer
        // need the expect since parse returns Result, or handle it via matching
        let guess:u32 = match guess.trim().parse() {
            Ok(the_number) => {
                if the_number == 1318 {
                    println!("SECRET EXIT ORDERED.");
                    break;
                } else {
                    the_number
                }
            },
            Err(_) => {
                println!("Could not convert the input to u32.");
                continue;
            }
        };

        // compare
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal => {
                println!("Spot on!");
                break;
            },
        }
    }
}