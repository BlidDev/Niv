use crate::{
    structs::{GError, ERROR, Type}, gerr
};

pub fn add(a : Type, b : Type) -> Result<Type, ERROR> {

    if let (Type::I32(n1), Type::I32(n2)) = (a.clone(),b.clone()) {
        return Ok(Type::I32(n1 + n2))
    }
    else if let (Type::F32(n1), Type::F32(n2)) = (a.clone(),b.clone()) {
        return Ok(Type::F32(n1 + n2))
    }

    gerr!("Error: Tyring to add [{:?}] and [{:?}]", a,b)
}

pub fn sub(a : Type, b : Type) -> Result<Type, ERROR> {

    if let (Type::I32(n1), Type::I32(n2)) = (a.clone(),b.clone()) {
        return Ok(Type::I32(n1 - n2))
    }
    else if let (Type::F32(n1), Type::F32(n2)) = (a.clone(),b.clone()) {
        return Ok(Type::F32(n1 - n2))
    }

    gerr!("Error: Tyring to sub [{:?}] and [{:?}]", a,b)
}

pub fn mul(a : Type, b : Type) -> Result<Type, ERROR> {

    if let (Type::I32(n1), Type::I32(n2)) = (a.clone(),b.clone()) {
        return Ok(Type::I32(n1 * n2))
    }
    else if let (Type::F32(n1), Type::F32(n2)) = (a.clone(),b.clone()) {
        return Ok(Type::F32(n1 * n2))
    }

    gerr!("Error: Tyring to mul [{:?}] and [{:?}]", a,b)
}

pub fn div(a : Type, b : Type) -> Result<Type, ERROR> {

    if let (Type::I32(n1), Type::I32(n2)) = (a.clone(),b.clone()) {
        if n2 == 0 {
            return gerr!("Error: Cannot divide by 0")
        }
        return Ok(Type::I32(n1 / n2))
    }
    else if let (Type::F32(n1), Type::F32(n2)) = (a.clone(),b.clone()) {
        if n2 == 0.0 {
            return gerr!("Error: Cannot divide by 0")
        }
        return Ok(Type::F32(n1 / n2))
    }

    gerr!("Error: Tyring to div [{:?}] and [{:?}]", a,b)
}

pub fn eql(a : Type, b : Type) -> Result<Type, ERROR> {

    match (&a,&b) {
        (Type::VOID(), Type::VOID()) => Ok(Type::BOOL(true)),
        (Type::I32(a),  Type::I32 (b)) => {Ok(Type::BOOL(*a == *b))},
        (Type::F32(a), Type::F32 (b))  => {Ok(Type::BOOL(*a == *b))},       
        (Type::BOOL(a), Type::BOOL(b)) => {Ok(Type::BOOL(*a == *b))},
        (Type::CHAR(a), Type::CHAR(b)) => {Ok(Type::BOOL(*a == *b))},
        (Type::STR(a), Type::STR (b))  => {Ok(Type::BOOL(*a == *b))},
         _=> gerr!("Error: unidentified types [{:?}, {:?}]", a, b)
    }
}


pub fn bigger(a : Type, b : Type) -> Result<Type, ERROR> {

    match (&a,&b) {
        (Type::VOID(), Type::VOID()) => Ok(Type::BOOL(true)),
        (Type::I32(a),  Type::I32 (b)) => {Ok(Type::BOOL(*a > *b))},
        (Type::F32(a), Type::F32 (b))  => {Ok(Type::BOOL(*a > *b))},       
        (Type::BOOL(a), Type::BOOL(b)) => {Ok(Type::BOOL(*a > *b))},
        (Type::CHAR(a), Type::CHAR(b)) => {Ok(Type::BOOL(*a > *b))},
        (Type::STR(a), Type::STR (b))  => {Ok(Type::BOOL(*a > *b))},
         _=> gerr!("Error: unidentified types [{:?}, {:?}]", a, b)
    }
}

pub fn smaller(a : Type, b : Type) -> Result<Type, ERROR> {

    match (&a,&b) {
        (Type::VOID(), Type::VOID()) => Ok(Type::BOOL(true)),
        (Type::I32(a),  Type::I32 (b)) => {Ok(Type::BOOL(*a < *b))},
        (Type::F32(a), Type::F32 (b))  => {Ok(Type::BOOL(*a < *b))},       
        (Type::BOOL(a), Type::BOOL(b)) => {Ok(Type::BOOL(*a < *b))},
        (Type::CHAR(a), Type::CHAR(b)) => {Ok(Type::BOOL(*a < *b))},
        (Type::STR(a), Type::STR (b))  => {Ok(Type::BOOL(*a < *b))},
         _=> gerr!("Error: unidentified types [{:?}, {:?}]", a, b)
    }
}

pub fn neql(a : Type, b : Type) -> Result<Type, ERROR> {

    match (&a,&b) {
        (Type::VOID(), Type::VOID()) => Ok(Type::BOOL(false)),
        (Type::I32(a),  Type::I32 (b)) => {Ok(Type::BOOL(*a != *b))},
        (Type::F32(a), Type::F32 (b))  => {Ok(Type::BOOL(*a != *b))},       
        (Type::BOOL(a), Type::BOOL(b)) => {Ok(Type::BOOL(*a != *b))},
        (Type::CHAR(a), Type::CHAR(b)) => {Ok(Type::BOOL(*a != *b))},
        (Type::STR(a), Type::STR (b))  => {Ok(Type::BOOL(*a != *b))},
         _=> gerr!("Error: unidentified types [{:?}, {:?}]", a, b)
    }
}
pub fn and(a : Type, b : Type) -> Result<Type, ERROR> {
    let (Type::BOOL(aa), Type::BOOL(bb)) = (&a,&b) else {
        return gerr!("Error: Unsuitable types for [and] [{:?} {:?}]", a, b);
    };

    Ok(Type::BOOL(*aa && *bb))
}

pub fn or(a : Type, b : Type) -> Result<Type, ERROR> {
    let (Type::BOOL(aa), Type::BOOL(bb)) = (&a,&b) else {
        return gerr!("Error: Unsuitable types for [or] [{:?} {:?}]", a, b);
    };

    Ok(Type::BOOL(*aa || *bb))
}
