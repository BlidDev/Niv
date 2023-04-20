use crate::{structs::{Globals, Type, ERROR, GError}, util::get_variable, gerr};



pub fn post(glb : &Globals) ->Result<Type, ERROR> { 
    println!("{:#?}", glb.stack);
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

    Ok(Type::VOID())
}
