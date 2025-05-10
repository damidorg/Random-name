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
            let tinput = &input.trim();
            let chin = &tinput[1..];
            names.push(chin.to_string());
            if input.trim() == "done" {
                break;
            }
        }
        let randomnumber = rand::rng().random_range(..names.len());
        println!("The winner is {}", names[randomnumber])
    }
}
