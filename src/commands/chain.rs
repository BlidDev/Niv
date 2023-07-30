use crate::{structs::{Type, ERROR, Roots, Globals, QueryW, Scope, GError}, canvas::Canvas, util::traverse, gerr, sgerr};

pub fn chain(args : Vec<Type>, roots : &Roots,glb : &mut Globals, qr : &QueryW, scp : &Scope,
    cnv : &mut Option<Canvas>
) ->Result<Type, ERROR> { 

    let mut v = vec![];

    for node in args {
        sgerr!(
            Type::NODE(node),
            node,
            "Error: non NODE argument in [chain]: [{:?}]", node
        );
        
        v.push(traverse(&node, roots, qr, glb, scp, cnv)?);
    }


    Ok(Type::LIST(v))
}

