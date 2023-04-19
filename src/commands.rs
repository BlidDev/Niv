use std::collections::HashMap;

use crate::{
    structs::{Type, ERROR, GError, Globals, Scope, Pass}, 
    gerr, util::{get_variable, is_destination, traverse_scope},
    ops::*,
};

type Fun  = HashMap<char, fn(Type,Type)-> Result<Type, ERROR>>;

pub fn calw(args : Vec<Type>, pass : Pass) -> Result<Type, ERROR> {

    cal(args, &pass)
}
pub fn cal(args : Vec<Type>, (glb, ..) : &Pass) ->Result<Type, ERROR> {

    let num1 = get_variable(&args[0], &glb.stack)?;
    let op   = get_variable(&args[1], &glb.stack)?;
    let num2 = get_variable(&args[2], &glb.stack)?;
    
    let dis1 = num1.dis();
    if dis1 != num2.dis() {
        return gerr!("Error: Trying to run [cal] on mismatching types [{:?}] and [{:?}]", num1, num2);
    }

    let Type::CHAR(ref t_op) = op else {
        return gerr!("Error: Trying to run [cal] with invalid operator [{:?}]", op)
    };

    let mut map = Fun::new();
    map.insert('+', add);
    map.insert('-', sub);
    map.insert('*', mul);
    map.insert('/', div);
    map.insert('=', eql);
    map.insert('!', neql);
    map.insert('&', and);
    map.insert('|', or);
    
    if !map.contains_key(t_op) {
        return gerr!("Error: Trying to run [cal] with invalid operator [{:?}]", op)
    }

    map[&t_op](num1, num2)
}



pub fn set(args : Vec<Type>, (glb,..) : Pass) ->Result<Type, ERROR> {

    let name = is_destination(&args[0], &glb.stack, "set")?;
    let v = args[1].clone();

    //println!("Setting [{}] = [{:?}]", name, v.clone());
    *glb.stack.entry(name).or_insert(v.clone()) = v.clone();

    Ok(args[1].clone())
}

pub fn op(args : Vec<Type>, pass : Pass) ->Result<Type, ERROR> {

    _ = is_destination(&args[0], &pass.0.stack, "op")?;
    let op = args[1].clone();
    let b  = args[2].clone();

    let des = args[0].clone();
    set(vec![des.clone(), cal(vec![des, op, b], &pass)?], pass)
}

pub fn post(_ : Vec<Type>, (glb, ..) : Pass) ->Result<Type, ERROR> { 
    println!("{:#?}", glb.stack);
    Ok(Type::VOID())
}

pub fn print(args : Vec<Type>, (glb, ..) : Pass) ->Result<Type, ERROR> { 
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




pub fn ifcommand(args : Vec<Type>, (glb, scp, qr) : Pass) ->Result<Type, ERROR> { 
    
    let val = cal(args, &(glb, scp, qr))?;
    let Type::BOOL(b) = &val else {
        return gerr!("Error: [if] check returned [{val:?} instead of BOOL]");
    };
    if !*b {return Ok(Type::VOID());}

    if let Some(scope) = scp.children.get(&(&glb.curr + 1)) {
        traverse_scope(scope,  qr, glb)?;
    }

    Ok(Type::VOID())
}

