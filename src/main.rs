use std::{io::{BufReader, BufRead}, fs::File};

mod sturcts;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let reader = BufReader::new(File::open("scripts/example.glg")?);

    let lines : Vec<String> = reader.lines().map(|l| l.unwrap().trim().to_string()).collect();

    let code : Vec<String> = lines.iter().filter(|l| 
        { 
            let l2 = (*l).clone();
            !(balanced_braces(&[l2], '[', ']').is_empty())
        }).map(|l| l.to_string()).collect();
    let code = code.iter().map(|l| balanced_braces(&[(*l).clone()], '[', ']')).collect::<Vec<Vec<String>>>();
    println!("{lines:#?}]\n\n");
    println!("{code:#?}]\n\n");
    Ok(())
}

fn balanced_braces(args: &[String], open: char, close: char) -> Vec<String> {
    let mut parts = Vec::new();

    for arg in args {
        if !arg.contains(open) {
            continue;
        }

        let mut chars = Vec::new();
        let mut n = 0;

        for c in arg.chars() {
            if c == open {
                if n > 0 {
                    chars.push(c);
                }
                n += 1;
            } else if c == close {
                n -= 1;
                if n > 0 {
                    chars.push(c);
                } else if n == 0 {
                    let part = chars.iter().collect::<String>().trim().to_string();
                    parts.push(part);
                    chars.clear();
                }
            } else if n > 0 {
                chars.push(c);
            }
        }
    }

    parts
}
