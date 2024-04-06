use crate::{canvas::Canvas, expression::{Expr, Registries}, gerr, run_command, structs::{parse_type, GError, Globals, NodeType, QueryW, Roots, Scope, Type, ERROR, Stack}, traverse};


#[derive(Debug)]
pub struct State {
    pub sequence : Vec<Expr>,
    pub registries : Registries
}

impl State {
    pub fn post(&self) {
        for (i, s) in self.sequence.iter().enumerate() {
            println!("{} - {s:?}", i)
        }
        println!("\n{:#?}", self.registries);
    }
    
}


pub fn traverse_state(state : &mut State, roots : &Roots,query : &QueryW, glb : &mut Globals, scope : &Scope,
    cnv : &mut Option<Canvas>
    ) -> Result<Type,ERROR> {
    for expr in state.sequence.iter() {
        let Expr::Command(cmd, e_args) = expr else {
            return gerr!("Error: expression is [{:?}] instead of command", expr);
        };
        let mut args = vec![];
        for a in e_args.iter() {
            let tmp = traverse_expression(&a, &mut state.registries, &glb.stack)?;
            args.push(tmp.clone());
        }
        let Type::STR(cmd) = traverse_expression(&cmd, &mut state.registries, &glb.stack)? else {
            return gerr!("Error: command is [{cmd:?}] instead of STR");
        };

        let tmp = run_command(roots, query, &cmd, args, glb, scope, cnv)?;
        if cmd == "set" {
            println!("-> {tmp:?}")
        }
        //state.registries.reset();
        //println!("{cmd} {args:?}");
        state.registries.put(tmp);
        state.registries.index = (state.registries.index + 1) % state.registries.len;
    }
    Ok(Type::VOID())
}

pub fn traverse_expression(expr : &Expr, reg : &mut Registries, stack : &Stack) -> Result<Type, ERROR> {
    match expr {
        Expr::Const(val) => Ok((*val).clone()),
        Expr::Carry(index) => {
            let tmp = reg.set.get(*index).unwrap().clone();
            if *index == 3 { println!("{:?}", reg.set) }
            Ok(tmp)
        }
        Expr::RawVariable(name) => if let Some(var) = stack.get(name) { return Ok(var.clone()) }
        else { gerr!("Error: cannot find variable [{}]", name) },
        _ => gerr!("Error: Unexpected command expression")
    }
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
