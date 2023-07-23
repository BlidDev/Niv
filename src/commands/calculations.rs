use std::collections::HashMap;

use crate::{structs::{Globals, Type, ERROR, GError}, util::{is_destination, get_variable}, gerr, ops::*};

use super::variables::set;

type Fun<'a>  = HashMap<&'a str, fn(Type,Type)-> Result<Type, ERROR>>;

pub fn cal(args : Vec<Type>, glb : &Globals) ->Result<Type, ERROR> {

    let num1 = get_variable(&args[0], &glb.stack)?;
    let op   = get_variable(&args[1], &glb.stack)?;
    let num2 = get_variable(&args[2], &glb.stack)?;
    
    let dis1 = num1.dis();
    if dis1 != num2.dis() {
        return gerr!("Error: Trying to run [cal] on mismatching types [{:?}] and [{:?}]", num1, num2);
    }

    let op : String = match op {
        Type::CHAR(ref t_op) => t_op.to_string(),
        Type::STR(ref t_op) => t_op.to_string(),

        _ =>return gerr!("Error: Trying to run [cal] with invalid operator [{:?}]", op),
    };

    let mut map = Fun::new();
    map.insert("+", add);
    map.insert("-", sub);
    map.insert("*", mul);
    map.insert("/", div);
    map.insert("=", eql);
    map.insert("!", neql);
    map.insert(">", bigger);
    map.insert("<", smaller);
    map.insert("&", and);
    map.insert("|", or);
    map.insert("^", pow);
    map.insert(">=", bigger_or_eql);
    map.insert("<=", smaller_or_eql);
    
    if !map.contains_key(op.as_str()) {
        return gerr!("Error: Trying to run [cal] with invalid operator [{:?}]", op)
    }

    map[op.as_str()](num1, num2)
}


pub fn op(args : Vec<Type>, glb : &mut Globals) ->Result<Type, ERROR> {

    let des = is_destination(&args[0], &glb.stack, "op")?; // var
    let op = args[1].clone();
    let b  = args[2].clone();

    let value = cal(vec![Type::STR(format!("${}", des.clone())), op, b], glb)?;

    set(vec![Type::STR(des.clone()), value], glb)
}


pub fn sqrt(args : Vec<Type>, glb : &Globals) -> Result<Type,ERROR> {
    let a = get_variable(&args.first().unwrap(), &glb.stack)?;

    match a {
        Type::I32(num) => {
            if num < 0 { return gerr!("Error: negative number given to [sqrt]: [{:?}]", num); }
            return Ok(Type::F32( (num as f32).sqrt() ))
        },
        Type::F32(num) => {
            if num < 0.0 { return gerr!("Error: negative number given to [sqrt]: [{:?}]", num); }
            return Ok(Type::F32( num.sqrt() ))
        },
       _ => return gerr!("Error: wrong argument given to [sqrt]: [{:?}]",a), 

    }
}


pub fn sin(args : Vec<Type>, glb : &Globals) -> Result<Type,ERROR> {
    let a = get_variable(&args.first().unwrap(), &glb.stack)?;

    match a {
        Type::I32(num) => {
            return Ok(Type::F32( (num as f32).to_radians().sin() ))
        },
        Type::F32(num) => {
            return Ok(Type::F32( num.to_radians().sin()))
        },
       _ => return gerr!("Error: wrong argument given to [sin]: [{:?}]",a), 

    }
}



pub fn cos(args : Vec<Type>, glb : &Globals) -> Result<Type,ERROR> {
    let a = get_variable(&args.first().unwrap(), &glb.stack)?;

    match a {
        Type::I32(num) => {
            return Ok(Type::F32( (num as f32).to_radians().cos() ))
        },
        Type::F32(num) => {
            return Ok(Type::F32( num.to_radians().cos()))
        },
       _ => return gerr!("Error: wrong argument given to [cos]: [{:?}]",a), 

    }
}



pub fn tan(args : Vec<Type>, glb : &Globals) -> Result<Type,ERROR> {
    let a = get_variable(&args.first().unwrap(), &glb.stack)?;

    match a {
        Type::I32(num) => {
            return Ok(Type::F32( (num as f32).to_radians().tan() ))
        },
        Type::F32(num) => {
            return Ok(Type::F32( num.to_radians().tan()))
        },
       _ => return gerr!("Error: wrong argument given to [tan]: [{:?}]",a), 

    }
}

pub fn asin(args : Vec<Type>, glb : &Globals) -> Result<Type,ERROR> {
    let a = get_variable(&args.first().unwrap(), &glb.stack)?;

    match a {
        Type::I32(num) => {
            return Ok(Type::F32( (num as f32).asin().to_degrees()))
        },
        Type::F32(num) => {
            return Ok(Type::F32( num.asin().to_degrees()))
        },
       _ => return gerr!("Error: wrong argument given to [asin]: [{:?}]",a), 

    }
}



pub fn acos(args : Vec<Type>, glb : &Globals) -> Result<Type,ERROR> {
    let a = get_variable(&args.first().unwrap(), &glb.stack)?;

    match a {
        Type::I32(num) => {
            return Ok(Type::F32( (num as f32).acos().to_degrees()))
        },
        Type::F32(num) => {
            return Ok(Type::F32( num.acos().to_degrees()))
        },
       _ => return gerr!("Error: wrong argument given to [acos]: [{:?}]",a), 

    }
}



pub fn atan(args : Vec<Type>, glb : &Globals) -> Result<Type,ERROR> {
    let a = get_variable(&args.first().unwrap(), &glb.stack)?;

    match a {
        Type::I32(num) => {
            return Ok(Type::F32( (num as f32).atan().to_degrees() ))
        },
        Type::F32(num) => {
            return Ok(Type::F32( num.atan().to_degrees()))
        },
       _ => return gerr!("Error: wrong argument given to [tan]: [{:?}]",a), 

    }
}
