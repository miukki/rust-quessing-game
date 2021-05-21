use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::mem;
use self::dusk::plonk;
use self::something::User; // Ok.

//use library
use guessing_game::GREETING;
use guessing_game::say_hi;

use dusk_plonk::prelude::*;

//plonk usage
mod dusk {
    pub fn plonk() {
        println!("load examples PLONK")
        let composer = &mut StandardComposer
    }
}

  
mod something {
    #[derive(Debug)]//#[derive(Debug, Default)]
    pub struct User {
        pub name: String,
        pub id: i32
    }

    
}
//cargo doc --open
//target/doc/rand/trait.Rng.html#method.gen_range

enum State {
    Locked,
    Failed,
    Unlocked
}

fn main() {

    let user = User { name: "Gary".to_string(), id: 1234 };
    println!("{:?}", user);
    println!("g {}", GREETING);

    plonk();


    //say_hi();
    //guessing_game()
    //another_function()
    //return_value()
    //size_exml()
    //for_loop();
    //match_with_range();
    //password();


}


fn password() {

    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        println!("Please enter code");
        match state {
            State::Locked => {
                let mut input = String::new();

                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());//handle input 
                    }
                    Err(_) => continue,
        
                };

                if entry == code {//code.starts_with(&entry)
                    state = State::Unlocked;
                    continue;
                };


                if !code.starts_with(&entry) {
                    state = State::Failed;
                    continue;
                };

                // println!("test")
            }
            State::Unlocked => {
                println!("Unlocked!");
                return;
            }
            State::Failed => {
                println!("Failed!");
                entry.clear();
                state = State::Locked;
                continue;
            }

        };
    };

}

fn match_with_range() {

    let _code = 7;

    let _code_c = match _code {
        7 => "rus",
        1..=1000 => "unknown", //inclusive to 1000
        _ => "invalid"
    };

    println!("code {}, {}", _code, _code_c);

}


fn guessing_game() {

    println!("Guess the number!");

    let secret_number_i32 = thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number_i32);

    let y: f64 = thread_rng().gen(); // generates a float between 0 and 1

    println!("char f64: {}", y);


    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // UTF-8 encoded bit of text. Static method

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        //Rust allows us to shadow the previous value of guess with a new one
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,//error handling https://doc.rust-lang.org/rust-by-example/error/result.html
        };
        
        //.parse().expect("Failed to read line");

        match guess.cmp(&secret_number_i32) {// Rust has a strong, static type system.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
    // let x = 5;
    // let y = 10;
    
    // println!("x = {} and y = {1}", x, y);
}


fn another_function() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

fn return_value() {
    let _x = 5;

    let y = {
        let _x = 3;
        _x + 1//If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.
    };

    println!("The value of y is: {}", y);
}



fn size_exml() {
    // let map: HashMap<u32, u32> = HashMap::new();
    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);
    println!("z {}, size_of_z {}-bytes, {}-bit", z,  mem::size_of_val(&z), size_of_z*8);

    let d: char = 'x';
    println!("d {}, size_of_z {}-bytes, {}-bit", d, mem::size_of_val(&d), mem::size_of_val(&d)*8);

    let b: bool = true;
    println!("b {}, size_of_z {}-bytes, {}-bit", b, mem::size_of_val(&b), mem::size_of_val(&b)*8);


}

fn for_loop() {

    for _x in 1..11 {
        if _x == 3 { continue; }
        println!("x = {}", _x)
    }

    for (pos,y) in (30..41).enumerate() {
        println!("pos = {}, y = {}", pos, y)
    }
}



