use sfml::window::sensor::Type;

use crate::{canvas::Canvas, expression::{Expr, Registries}, gerr, run_command, structs::{parse_type, Globals, NodeType, QueryW, Roots, Scope, ERROR}, traverse};


pub struct State {
    pub sequence : Vec<Expr>,
    pub registries : Registries
}


pub fn traverse_state(state : &mut State, roots : &Roots,query : &QueryW, glb : &mut Globals, scope : &Scope,
    cnv : &mut Option<Canvas>
    ) -> Result<Type,ERROR> {
    for expr in state.sequence.iter() {

    }
    todo!()
}

    /*

    match node {
        NodeType::Value(value) => {
            Ok(parse_type(value, roots, query, glb, scope, cnv)?)
        }

        NodeType::Nested(command, childern) => {
            let command = traverse(command, roots, query, glb, scope, cnv)?;
            let Type::STR(ref name) = command else {
                return gerr!("Error: Command is [{command:?}] instead of STR");
            };

            if "while" == name {
                return run_command(roots,query, &name, vec![Type::NODE(childern[0].clone())], glb, scope, cnv)
            }

            if "chain" == name {
                let v : Vec<Type> = childern.iter().map(|e| Type::NODE(e.clone())).collect();
                return run_command(roots, query, &name, v, glb, scope, cnv)
            }

            if name == "singleif" {
                let mut args : Vec<Type> = vec![];
                let offset = match childern.len() {
                    4 => 1,
                    5 => 2,
                    _ => return gerr!("Error: [singleif] takes [4] or [5] arguments but [{}] were provided"
                , childern.len())
                };
                for i in 0..childern.len() - offset  {
                    args.push(traverse(&childern[i], roots, query, glb, scope, cnv)?);
                }
                for i in 1..offset+1 {
                    args.push(Type::NODE(childern.get(2 + i).unwrap().clone()));
                }
                return run_command(roots, query, &name, args, glb, scope, cnv)
            }

            let mut args : Vec<Type> = vec![];
            for child in childern  {
                args.push(traverse(&child, roots,  query, glb, scope, cnv)?);
            }

            //println!("{command:?} {args:?}\n");
            run_command(roots, query, &name, args, glb, scope, cnv)
        },
    }
     */
