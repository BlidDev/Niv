use std::collections::HashMap;

use crate::{structs::{Globals, Type, ERROR, GError}, util::{is_destination, get_variable}, gerr, ops::*};

use super::variables::set;

type Fun  = HashMap<char, fn(Type,Type)-> Result<Type, ERROR>>;

pub fn cal(args : Vec<Type>, glb : &Globals) ->Result<Type, ERROR> {

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
    map.insert('>', bigger);
    map.insert('<', smaller);
    map.insert('&', and);
    map.insert('|', or);
    
    if !map.contains_key(t_op) {
        return gerr!("Error: Trying to run [cal] with invalid operator [{:?}]", op)
    }

    map[&t_op](num1, num2)
}


pub fn op(args : Vec<Type>, glb : &mut Globals) ->Result<Type, ERROR> {

    let des = is_destination(&args[0], &glb.stack, "op")?; // var
    let op = args[1].clone();
    let b  = args[2].clone();

    let value = cal(vec![Type::STR(format!("${}", des.clone())), op, b], glb)?;

    set(vec![Type::STR(des.clone()), value], glb)
}
