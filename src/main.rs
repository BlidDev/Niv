use std::{io::{BufReader, BufRead}, fs::File};

mod util;
mod structs;
mod macros;
mod ops;
mod commands;

use structs::{Stack, Globals, QueryW};
use util::*;
use commands::*;
use crate::structs::CommandQuery;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let reader = BufReader::new(File::open("scripts/example.glg")?);


    let lines : Vec<String> = reader.lines().map(|l| l.unwrap().trim().to_string()).collect();

    let roots = find_root_scopes(&lines)?;

    let main = root_to_scope_tree(&lines, 
        &roots.get("MAIN").expect("Error: Root scope [MAIN] not found"))?;

    let _code : Vec<String> = lines.iter().filter(|l| 
        { 
            let l2 = (*l).clone();
            !(balanced_braces(&[l2], '[', ']').is_empty())
        }).map(|l| l.to_string()).collect();

    //println!("\nCall Order:\n");

    let mut query = CommandQuery::new();

    commands![
        (query),
        {
            set =>      (set, Some(2)),
            release => (release, Some(1)),
            reset => (reset, Some(0)),

            cal =>      (calw,Some(3)),
            op =>       (op,Some(3)),

            post =>     (post,Some(0)),
            print =>    (print,None),

            if =>       (ifcommand,Some(3)),
            while =>       (whilecommand,Some(1))
        }
    ];
    
    let mut glb = Globals {
        stack : Stack::default(),
        curr : 0,
    };
    traverse_scope(&main, &QueryW(query.clone()), &mut glb)?;

    Ok(())
}

