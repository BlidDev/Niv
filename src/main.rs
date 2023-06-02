use std::{io::{BufReader, BufRead}, fs::File, rc::Rc};

mod util;
mod structs;
mod macros;
mod ops;
pub mod commands;

use structs::{Stack, Globals, QueryW};
use util::*;
use commands::wrappers::*;
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
            set =>      (set_w, Some(2)),
            release =>  (release_w, Some(1)),
            reset =>    (reset_w, Some(0)),

            cal =>      (cal_w,Some(3)),
            op =>       (op_w,Some(3)),

            post =>     (post_w,Some(0)),
            print =>    (print_w,None),
            input =>    (input_w,None),
            inputcast =>(inputcast_w,None),

            if =>       (ifcommand_w,Some(3)),
            while =>    (whilecommand_w,Some(1))
        }
    ];


    
    let mut glb = Globals {
        stack : Stack::default(),
        curr : 0,
        s : " "
    };
    let mut ctx = None;
    let ctr = Rc::new(None);
    let cnv = Rc::new(None);
    traverse_scope(&main, &QueryW(query.clone()), &mut glb, &mut ctx, ctr, cnv)?;

    Ok(())
}


