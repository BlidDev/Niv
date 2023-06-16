use crate::{structs::{Globals, Type, ERROR, GError, QueryW, Scope}, util::{get_variable, traverse_scope, traverse}, gerr, canvas::Canvas};

use sdl2::{render::TextureCreator, video::WindowContext};
use std::rc::Rc;
use super::calculations::cal;

pub fn ifcommand(args : Vec<Type>, glb : &mut Globals, qr : &QueryW, scp : &Scope,
    cnv : &mut Option<Canvas>
    ) ->Result<Type, ERROR> { 
    
    let val = cal(args, glb)?;
    let Type::BOOL(b) = &val else {
        return gerr!("Error: [if] check returned [{val:?} instead of BOOL]");
    };
    if !*b {return Ok(Type::VOID());}

    if let Some(scope) = scp.children.get(&(&glb.curr + 1)) {
        traverse_scope(scope,  qr, glb, cnv)?;
    }

    Ok(Type::VOID())
}

pub fn whilecommand(args : Vec<Type>, glb : &mut Globals, qr : &QueryW, scp : &Scope,
    cnv : &mut Option<Canvas>
    ) ->Result<Type, ERROR> { 
    
    let Type::NODE(ref node) = args[0] else {
        return gerr!("Error: [while] need NODE as argument but got {:?} instead", args[0])
    };
    let Type::BOOL(ref b) = get_variable(&traverse(node, qr, glb, scp, cnv)?, &glb.stack)? else {
        return gerr!("Error: [while] check returned [{:?}] instead of BOOL]",
            args[0]);
    };
    if !*b {return Ok(Type::VOID());}

    let curr = glb.curr;
    while *b {
        let Type::BOOL(ref b) = get_variable(&traverse(node, qr, glb, scp, cnv)?, &glb.stack)? else {
            return gerr!("Error: [while] check returned [{:?}] instead of BOOL]",
                args[0]);
        };
        if !*b {return Ok(Type::VOID());}

        if let Some(scope) = scp.children.get(&(&glb.curr + 1)) {
            traverse_scope(scope,  qr, glb, cnv)?;
        }
        glb.curr = curr;
    }

    Ok(Type::VOID())
}

