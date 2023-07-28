use std::str::FromStr;

use crate::{structs::{Type, ERROR, Globals, GError, TypeIndex}, sgerr, util::args_to_vars, gerr};



pub fn cst(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR>{

    let args = args_to_vars(&args, &glb.stack)?;

    let typename = &args[0];
    sgerr!(
        Type::STR(ref typename),
        typename,
        "Error: invalid typename given to [cst]: [{:?}]", typename
    );

    let var = args[1].clone();
    
    let typename = TypeIndex::from_str(&typename)?;

    if var.dis() == typename as usize {
        return Ok(var);
    }

    match typename {
        TypeIndex::VOID => { Ok(Type::VOID())}, 
        TypeIndex::I32  => { Ok(Type::I32 (type_to_i32(var)?)) },
        TypeIndex::F32  => { Ok(Type::F32 (type_to_f32(var)?)) },
        TypeIndex::BOOL => { Ok(Type::BOOL(type_to_bool(var)?)) },
        TypeIndex::CHAR => { Ok(Type::CHAR(type_to_char(var)?)) },
        TypeIndex::STR  => { Ok(Type::STR(var.to_string()?)) },
        TypeIndex::LIST => { Ok(Type::LIST(type_to_list(var)?)) },
        TypeIndex::UTYPE=> {return gerr!("Error: cannot cast types into type [{:?}]", typename)},
        TypeIndex::NODE => {return gerr!("Error: cannot cast types into type [{:?}]", typename)}, 
    }

}


fn type_to_i32(var : Type) -> Result<i32, ERROR> {

    match var {
        Type::F32(v)  => Ok(v.round() as i32),
        Type::BOOL(v) => Ok(v as i32),
        Type::CHAR(v) => Ok(v as i32),
        Type::STR(v) => Ok(i32::from_str(&v)?),
        _ => gerr!("Error: cannot cast variable of type [{:?}] into I32", var)
    }
}

fn type_to_f32(var : Type) -> Result<f32, ERROR> {

    match var {
        Type::I32(v)  => Ok(v as f32),
        Type::CHAR(v)  => Ok(v as u8 as f32),
        Type::STR(v) => Ok(f32::from_str(&v)?),
        _ => gerr!("Error: cannot cast variable of type [{:?}] into F32", var)
    }
}


fn type_to_bool(var : Type) -> Result<bool, ERROR> {

    match var {
        Type::I32(v)  => {
            match v {
                0 => Ok(false),
                1 => Ok(true),
                _ => gerr!("Error: invalid value when trying to cast [{:?}] into bool, valid values are: [0, 1]", var)
            }
        },
        Type::STR(v) => Ok(bool::from_str(&v)?),
        _ => gerr!("Error: cannot cast variable of type [{:?}] into BOOL", var)
    }
}

fn type_to_char(var : Type) -> Result<char, ERROR> {

    match var {
        Type::I32(v) => Ok(v as u8 as char),
        Type::STR(v) => Ok(char::from_str(&v)?),
        _ => gerr!("Error: cannot cast variable of type [{:?}] into CHAR", var)
    }
}

fn type_to_list(var : Type) -> Result<Vec<Type>, ERROR> {

    match var {
        Type::STR(v) => Ok(v.chars().map(|c| Type::CHAR(c)).collect()),
        _ => gerr!("Error: cannot cast variable of type [{:?}] into CHAR", var)
    }
}
