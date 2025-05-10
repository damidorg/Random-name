use rand::Rng;
use std::io;
fn main() {
    loop {
        println!("write a list of names to pick a random one (type 'done' when finished)");
        let mut names: Vec<String> = Vec::new();
        loop {
            println!(">");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("invalid input");
            if input.trim() == "done" {
                break;
            }
            if input == "\n" {
                println!("input a name");
            }
            if input == "\n\r" {
                println!("input a name");
            } else {
                let chin = input.trim();
                names.push(chin.to_string());
            }
        }

        if names.len() > 0 {
            let randomnumber = rand::rng().random_range(..names.len());
            println!("The winner is {}", names[randomnumber])
        } else {
            println!("no names")
        }
    }
}
