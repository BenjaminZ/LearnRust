use std::collections::HashMap;
use std::io;

fn main() {
    let mut map: HashMap<String, String> = HashMap::new();
    loop {
        println!("Please type in a command");
        println!("Format should be 'Add * to *'");
        println!("Type show to see result");

        // read from input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input.to_lowercase() == "show" {
            break;
        }

        // deconstruct command
        let input: Vec<&str> = input.split(' ').collect();
        if input.len() != 4 {
            println!("Wrong format");
            continue;
        }
        for word in input.iter() {
            if word.is_empty() {
                println!("Wrong format");
                continue;
            }
        }
        if input[0].to_lowercase() != "add" {
            println!("Wrong format");
            continue;
        }
        if input[2].to_lowercase() != "to" {
            println!("Wrong format");
            continue;
        }

        let name = String::from(input[1]);
        let department = String::from(input[3]);
        map.insert(name, department);
    }

    if !map.is_empty() {
        println!("{:?}", map);
    }

    println!("No employee added");
}
