use std::collections::HashMap;

use crate::{canvas::Canvas, expression::{Expr, Registries}, gerr, run_command, structs::{GError, Globals, QueryW, Roots, Scope, Type, ERROR, Stack}};


#[derive(Debug)]
pub struct State {
    //pub sequence : Vec<Expr>,
    pub roots : HashMap<String, Scope>
    pub registries : Registries
}

impl State {
    #[allow(unused)]
    pub fn post(&self) {
        for (i, s) in self.sequence.iter().enumerate() {
            println!("{} - {s:?}", i)
        }
        println!("\n{:#?}", self.registries);
    }
    
}

pub fn traverse_state(state : &mut State, main : String, roots : &Roots,query : &QueryW, glb : &mut Globals, scope : &Scope,
    cnv : &mut Option<Canvas>
    ) -> Result<Type,ERROR> {

    if let 

    todo!()
}
pub fn traverse_sequence(state : &mut State, roots : &Roots,query : &QueryW, glb : &mut Globals, scope : &Scope,
    cnv : &mut Option<Canvas>
    ) -> Result<Type,ERROR> {
    state.registries.reset();
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
        state.registries.put(tmp);
        state.registries.index = (state.registries.index + 1) % state.registries.len;
    }
    Ok(Type::VOID())
}

pub fn traverse_expression(expr : &Expr, reg : &mut Registries, stack : &Stack) -> Result<Type, ERROR> {
    match expr {
        Expr::Const(val) => Ok((*val).clone()),
        Expr::Carry(index) => {
            let tmp = reg.set.get( (*index) % reg.len ).unwrap().clone();
            Ok(tmp)
        }
        Expr::RawVariable(name) => if let Some(var) = stack.get(name) { return Ok(var.clone()) }
        else { gerr!("Error: cannot find variable [{}]", name) },
        _ => gerr!("Error: Unexpected command expression")
    }
}
