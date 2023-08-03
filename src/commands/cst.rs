use std::str::FromStr;

use crate::{structs::{Type, ERROR, Globals, GError, TypeIndex, Scope, QueryW, Roots}, sgerr, util::{args_to_vars, traverse, make_tree}, gerr, canvas::Canvas};



pub fn cst(args : Vec<Type>, roots : &Roots,glb : &mut Globals, qr : &QueryW, scp : &Scope,
    cnv : &mut Option<Canvas>
) -> Result<Type, ERROR>{

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

    let res : Result<Type, ERROR>= match typename {
        TypeIndex::VOID => { Ok(Type::VOID())}, 
        TypeIndex::I32  => { type_to_i32(&var) },
        TypeIndex::F32  => { type_to_f32(&var) },
        TypeIndex::BOOL => { type_to_bool(&var) },
        TypeIndex::CHAR => { type_to_char(&var) },
        TypeIndex::STR  => { Ok(Type::STR(var.to_string()?)) },
        TypeIndex::LIST => { type_to_list(&var) },
        TypeIndex::UTYPE=> {return gerr!("Error: cannot cast types into type [{:?}]", typename)},
        TypeIndex::NODE => {return gerr!("Error: cannot cast types into type [{:?}]", typename)}, 
        TypeIndex::RETURN => {Ok(Type::RETURN(Box::new(var.clone())))}
    };

    match res {
        Ok(r) => return Ok(r),

        Err(e) => {
            let Type::STR(var) = var else { return gerr!("Error: cannot convert [{:?}] to [{:?}] - [{}]", var, typename, e)};
            let Ok(t) = traverse(&make_tree(&var, true)?, roots, qr, glb, scp, cnv) else {
                return gerr!("Error: cannot convert [{:?}] to [{:?}] - [{}]", var, typename, e)
            };

            if t.dis() != typename as usize { return gerr!("Error: cannot convert [{:?}] to [{:?}] - [{}]", var, typename, e)}

            Ok(t)
        }
        
    }

}


fn type_to_i32(var : &Type) -> Result<Type, ERROR> {

    match var {
        Type::F32(v)  => Ok(Type::I32(v.round() as i32)),
        Type::BOOL(v) => Ok(Type::I32(*v as i32)),
        Type::CHAR(v) => Ok(Type::I32(*v as i32)),
        Type::STR(v) => Ok(Type::I32(i32::from_str(&v)?)),
        _ => gerr!("Error: cannot cast variable of type [{:?}] into I32", var)
    }
}

fn type_to_f32(var : &Type) -> Result<Type, ERROR> {

    match var {
        Type::I32(v)   => Ok(Type::F32(*v as f32)),
        Type::CHAR(v)  => Ok(Type::F32(*v as u8 as f32)),
        Type::STR(v) => Ok(Type::F32(f32::from_str(&v)?)),
        _ => gerr!("Error: cannot cast variable of type [{:?}] into F32", var)
    }
}


fn type_to_bool(var : &Type) -> Result<Type, ERROR> {

    match var {
        Type::I32(v)  => {
            match v {
                0 => Ok(Type::BOOL(false)),
                1 => Ok(Type::BOOL(true)),
                _ => gerr!("Error: invalid value when trying to cast [{:?}] into bool, valid values are: [0, 1]", var)
            }
        },
        Type::STR(v) => Ok(Type::BOOL(bool::from_str(&v)?)),
        _ => gerr!("Error: cannot cast variable of type [{:?}] into BOOL", var)
    }
}

fn type_to_char(var : &Type) -> Result<Type, ERROR> {

    match var {
        Type::I32(v) => Ok(Type::CHAR(*v as u8 as char)),
        Type::STR(v) => Ok(Type::CHAR(char::from_str(&v)?)),
        _ => gerr!("Error: cannot cast variable of type [{:?}] into CHAR", var)
    }
}

fn type_to_list(var : &Type) -> Result<Type, ERROR> {

    match var {
        Type::STR(v) => Ok(Type::LIST(v.chars().map(|c| Type::CHAR(c)).collect())),
        _ => gerr!("Error: cannot cast variable of type [{:?}] into CHAR", var)
    }
}
