use std::io::Write;

use crate::{structs::{Globals, Type, ERROR, GError, TypeIndex, parse_type}, util::get_variable, gerr};



pub fn post(glb : &Globals) ->Result<Type, ERROR> { 
    println!("{:#?}", glb.stack);
    std::io::stdout().flush()?;
    Ok(Type::VOID())
}

pub fn print(args : Vec<Type>, glb : &Globals) ->Result<Type, ERROR> { 
    if args.len() < 1 { 
        return gerr!("Error: No arguments given to [print]");
    }

    let Type::STR(format) = args[0].clone() else {
        return gerr!("Error: Invalid format in [print] [{:?}]", args[0].clone());
    };

    let matches = format.matches("{}").count();
    if matches < args.len() - 1 {
        return gerr!("Error: [{:?}] positionals were given in [print] but only [{}] provided", matches, args.len() - 1);
    }

    let mut format = format.clone();
    for i in 0..matches {
        let val = get_variable(&args[i + 1], &glb.stack)?;
        format = format.replacen("{}", &format!("{}", val),1);
    }
    
    print!("{}", format);
    std::io::stdout().flush()?;

    Ok(Type::VOID())
}

pub fn input(args : Vec<Type>, glb : &Globals) -> Result<Type, ERROR> {
    if !args.is_empty() {
        print(args.clone(), glb)?;
    }

    let mut line = String::new();

    _ = std::io::stdin().read_line(&mut line)?;

    line.pop();
    Ok(Type::STR(line))
}



pub fn inputcast(args : Vec<Type>, glb : &Globals) -> Result<Type, ERROR> {
    if args.len() > 1 {
        print(args[1..].to_vec(), glb)?;
    }

    let Type::STR(index) = get_variable(&args[0], &glb.stack)? else {
        return gerr!("Error: {:?} is not a valid TypeIndex", args[0])
    };


    let index = index.parse::<TypeIndex>()?;

    let mut line = String::new();
    _ = std::io::stdin().read_line(&mut line)?;
    line.pop();

    let var = parse_type(&line);

    if var.dis() != index.clone() as usize {
        return gerr!("Error: Could not parse {line} as {:?}, got {var:?} instead", index);
    }

    Ok(var)
}
