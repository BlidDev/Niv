use std::{io::{BufReader, BufRead}, fs::File};

mod util;
mod sturcts;

use sturcts::*;
use util::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let reader = BufReader::new(File::open("scripts/example.glg")?);

    let lines : Vec<String> = reader.lines().map(|l| l.unwrap().trim().to_string()).collect();

    let code : Vec<String> = lines.iter().filter(|l| 
        { 
            let l2 = (*l).clone();
            !(balanced_braces(&[l2], '[', ']').is_empty())
        }).map(|l| l.to_string()).collect();
    //let code = code.iter().map(|l| balanced_braces(&[(*l).clone()], '[', ']')).collect::<Vec<Vec<String>>>();

    /*let mut main = Scope::default();
    let mut counter : usize = 1;
    for command in code.iter() {
        if command.len() == 1 {
            let node = Node {
                command : command[0].clone(),
                id : counter,
                parent : 0,
                childern : vec![]
            };
            main.nodes.insert(counter, node);
            counter = counter + 1;
            continue;
        }
        for i in 1..command.len() {

        }

    }*/
    println!("{code:#?}]\n\n");
    let mut map = Scope::default();
    make_tree(&mut map, &code[2]);
    println!("{map:#?}");
    Ok(())
}

