use std::{io, string};
use rand::{prelude::*, rng};

fn main() {
    let guess_list = ["grapes", "banana", "orange", "mango"];
    let mut rng= rng();
    let index = rng.random_range(0..guess_list.len());

    let random_fruit = guess_list[index];
    println!("random_fruit: {}", random_fruit);

    let mut input = String::new();

    loop {
        match io::stdin().read_line(& mut input) {
        Ok(_) => {
            let fruit_selected = input.trim().to_lowercase();
            println!("fruit_selected {}", fruit_selected);

            if !guess_list.contains(&&fruit_selected.as_str()) {
                println!("Fruit doesn't exist");
                continue;
            }

            if guess_checker(&fruit_selected, random_fruit){
                println!("You are winner!");
                break;
            } else {
                println!("Try again");
            }
        }
        Err(err)=>{
            println!("Error {}", err)
        }
    }
    }
}

fn guess_checker(str1: &str, str2: &str) -> bool{
    return str1 == str2;
}
