use crate::{structs::{Globals, Type, ERROR, GError, QueryW, Scope, Roots}, util::{get_variable, traverse_scope, traverse, args_to_vars, traverse_root_scope}, gerr, canvas::Canvas, sgerr, commands::variables::{set, release}};

use super::calculations::cal;

pub fn ifcommand(args : Vec<Type>, roots : &Roots,glb : &mut Globals, qr : &QueryW, scp : &Scope,
    cnv : &mut Option<Canvas>
    ) ->Result<Type, ERROR> { 
    
    let val = cal(args, glb)?;
    let Type::BOOL(b) = &val else {
        return gerr!("Error: [if] check returned [{val:?} instead of BOOL]");
    };
    if !*b {return Ok(Type::VOID());}

    if let Some(scope) = scp.children.get(&(&glb.curr + 1)) {
        traverse_scope(roots, scope, qr, glb, cnv)?;
    }

    Ok(Type::VOID())
}

pub fn single_if(args : Vec<Type>, roots : &Roots, glb : &mut Globals, qr : &QueryW, scp : &Scope, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {

    let Some(Type::NODE(ref node)) = args.last() else {
        return gerr!("Error: [singleif] need NODE as argument but got {:?} instead", args[0])
    };
    let v = cal(args[0..3].to_vec(), glb)?;
    let Type::BOOL(ref b) = v else {
        return gerr!("Error: [singleif] check returned [{:?}] instead of BOOL]",
            v);
    };

    if !b { return Ok(Type::VOID()); }


    traverse(node, roots, qr, glb, scp, cnv)
}

pub fn whilecommand(args : Vec<Type>, roots : &Roots, glb : &mut Globals, qr : &QueryW, scp : &Scope, cnv : &mut Option<Canvas>) ->Result<Type, ERROR> { 
    
    let Type::NODE(ref node) = args[0] else {
        return gerr!("Error: [while] need NODE as argument but got {:?} instead", args[0])
    };
    let Type::BOOL(ref b) = get_variable(&traverse(node, roots, qr, glb, scp, cnv)?, &glb.stack)? else {
        return gerr!("Error: [while] check returned [{:?}] instead of BOOL]",
            args[0]);
    };
    if !*b {return Ok(Type::VOID());}

    let curr = glb.curr;
    while *b {
        let Type::BOOL(ref b) = get_variable(&traverse(node, roots, qr, glb, scp, cnv)?, &glb.stack)? else {
            return gerr!("Error: [while] check returned [{:?}] instead of BOOL]",
                args[0]);
        };
        if !*b {return Ok(Type::VOID());}

        if let Some(scope) = scp.children.get(&(&glb.curr + 1)) {
            traverse_scope(roots, scope, qr, glb, cnv)?;
        }
        glb.curr = curr;
    }

    Ok(Type::VOID())
}


pub fn run(args : Vec<Type>, roots : &Roots, glb : &mut Globals, qr : &QueryW, cnv : &mut Option<Canvas>) ->Result<Type, ERROR> { 
    
    let args = args_to_vars(&args, &glb.stack)?;

    if args.len() < 1 { return gerr!("Error: no arguments given to [run]") }

    sgerr!(
        Type::STR(ref name),
        args[0],
        "Error: invalid root scope name in [run], args are: [{:?}]", args
    );

    let list = glb.args_list.clone();
    let entry = list.get(name).expect(&format!("Error: no arguments entry for root scope [{}]", name));
    
    if entry.len() != args.len() -1 {
        return gerr!("Error: root scope [{}] requires [{}] arguments but [{}] were supplied", name, entry.len(),args.len() -1)
    }

    for (i, n) in entry.iter().enumerate() {
        set(vec![Type::STR(n.clone()), args[i + 1].clone()], glb)?;
    }

    traverse_root_scope(name, roots, qr, glb, cnv)?;

    for n in entry.iter() {
        release(vec![Type::STR(n.clone())], glb)?;
    }

    Ok(Type::VOID())
}
