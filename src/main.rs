// #![allow(unused)]
// static A:i16 = 13421;
// fn main() {
//     // let a: String= String::from("Hi i am global variable");
//     // let a: &str= "Hello, World!";
//     println!("{B}");
// }

// const B:&str = "Hi i am global variable";
// use std::io;

// fn main() {
//     println!("Guess the number!");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {guess}");
// }

// use std::cmp::Ordering;
// use std::io;

// use rand::random_range;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = random_range(1..=100);

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guesss: u32 = match guess.trim().parse() {
//             Ok(n) => n,
//             Err(_) => continue,
//         };

//         println!("You guessed: {guesss}");

//         match guesss.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }

fn main() {
    let mut counter = 0;
   

    let result = loop {
        counter += 1;
 println!("{counter}");
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}