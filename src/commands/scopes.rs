use crate::{structs::{Globals, Type, ERROR, GError, QueryW, Scope, Roots, NodeType}, util::{get_variable, traverse_scope, traverse, args_to_vars, traverse_root_scope}, gerr, canvas::Canvas, sgerr, commands::variables::{set, release}};

use super::calculations::cal;

pub fn ifcommand(args : Vec<Type>, roots : &Roots,glb : &mut Globals, qr : &QueryW, scp : &Scope,
    cnv : &mut Option<Canvas>
    ) ->Result<Type, ERROR> { 
    
    let val = cal(args, glb)?;
    let Type::BOOL(b) = &val else {
        return gerr!("Error: [if] check returned [{val:?} instead of BOOL]");
    };

    let Some(scope) = scp.children.get(&(&glb.curr + 1)) else {
        return gerr!("Error: could not find scope for [if]");
    };
    if !*b {
       return handle_else(glb.curr + scope.nodes.len() + 3,roots, glb, qr, scp, cnv) 
    }
    let curr = glb.curr;
    glb.curr = 0;
    let t = traverse_scope(roots, scope, qr, glb, cnv)?;
    glb.curr = curr;

    Ok(t)
}

pub fn single_if(args : Vec<Type>, roots : &Roots, glb : &mut Globals, qr : &QueryW, scp : &Scope, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {

    let (t_node, f_node) : (&Type, Option<&Type>) = {
        match args.len() {
            4 => (args.last().unwrap(), None),
            5 => (args.get(3).unwrap(), args.last()),
            _ => return gerr!("Error: [singleif] takes [4] or [5] arguments but [{}] were provided"
                , args.len())
        }

    };

    let Type::NODE(node) = t_node else {
        return gerr!("Error: [singleif] need NODE as argument but got {:?} instead", t_node)
    };
    let v = cal(args[0..3].to_vec(), glb)?;
    let Type::BOOL(ref b) = v else {
        return gerr!("Error: [singleif] check returned [{:?}] instead of BOOL]",
            v);
    };

    if !b {
        if let Some(f_node) = f_node {
            let Type::NODE(node) = f_node else {
                return gerr!("Error: [singleif] need NODE as argument but got {:?} instead", f_node)
            };
            return traverse(node, roots, qr, glb, scp, cnv)
        }
        return Ok(Type::VOID());
    }


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
            glb.curr = 0;
            let tmp = traverse_scope(roots, scope, qr, glb, cnv)?;
            match tmp {
                Type::VOID() => {},
                _ => {
                    glb.curr = curr;
                    return Ok(tmp)
                }
            }
        }
        else {
            return gerr!("Error: could not find scope for [while] command");
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

    let t = traverse_root_scope(name, roots, qr, glb, cnv)?;

    for n in entry.iter() {
        release(vec![Type::STR(n.clone())], glb)?;
    }

    Ok(t)
}


pub fn handle_else(index : usize, roots : &Roots,glb : &mut Globals, qr : &QueryW, scp : &Scope,
    cnv : &mut Option<Canvas>
) -> Result<Type, ERROR> {


    if index >= scp.nodes.len() {return Ok(Type::VOID())}

    let Some(Some(NodeType::Nested(first,children))) = scp.nodes.get(index) else {
        return Ok(Type::VOID())
    };

    match children.len() {
        0 => {
            let Type::STR(s) =  traverse(first, roots, qr, glb, scp, cnv)? else {
                return Ok(Type::VOID())
            };
            if &s != "else" {return Ok(Type::VOID());}

            let Some(scope) = scp.children.get(&(index + 1)) else {
                return gerr!("Error: could not find scope for [else]");
            };

            traverse_scope(roots, scope, qr, glb, cnv)
        },
        _ => {
            let first   = traverse(first, roots, qr, glb, scp, cnv)?;
            let second  = traverse(children.first().unwrap(), roots, qr, glb, scp, cnv)?;
            let (Type::STR(first), Type::STR(second)) = (first, second) else {
                return Ok(Type::VOID())
            };
            if (first, second) != ("else".to_string(), "if".to_string()) {return Ok(Type::VOID())}
            glb.curr = index;

            let args : Vec<Type> = {
                let mut v = vec![];
                let children = &children[1..].to_vec();
                for b in  children{
                    let t = traverse(b, roots, qr, glb, scp, cnv)?;
                    v.push(t);
                }
                v
            };

            if args.len() != 3 {
                return gerr!("Error: the command [if] requires [3] arguments but [{}] were provided"
                    ,args.len());
            }

            ifcommand(args,roots, glb, qr, scp, cnv)
        },
    }

    
}

