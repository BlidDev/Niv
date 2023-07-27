use crate::{
    structs::{GError, ERROR, Type}, gerr
};


#[allow(dead_code)]
pub fn add(a : Type, b : Type) -> Result<Type, ERROR> {
    match (&a, &b) {
        (Type::I32(n1), Type::I32(n2)) => return Ok(Type::I32(n1 + n2)),
        (Type::F32(n1), Type::F32(n2)) => return Ok(Type::F32(n1 + n2)),
        _ => gerr!("Error: Tyring to add [{:?}] and [{:?}]", a,b)
    }
}

#[allow(dead_code)]
pub fn sub(a : Type, b : Type) -> Result<Type, ERROR> {
    match (&a, &b) {
        (Type::I32(n1), Type::I32(n2)) => return Ok(Type::I32(n1 - n2)),
        (Type::F32(n1), Type::F32(n2)) => return Ok(Type::F32(n1 - n2)),
        _ => gerr!("Error: Tyring to sub [{:?}] and [{:?}]", a,b)
    }
}

#[allow(dead_code)]
pub fn mul(a : Type, b : Type) -> Result<Type, ERROR> {
    match (&a, &b) {
        (Type::I32(n1), Type::I32(n2)) => return Ok(Type::I32(n1 * n2)),
        (Type::F32(n1), Type::F32(n2)) => return Ok(Type::F32(n1 * n2)),
        _ => gerr!("Error: Tyring to mul [{:?}] and [{:?}]", a,b)
    }
}

#[allow(dead_code)]
pub fn div(a : Type, b : Type) -> Result<Type, ERROR> {

    match (&a, &b) {
        (Type::I32(n1), Type::I32(n2)) => {
            if *n2 == 0 {
                return gerr!("Error: Cannot divide by 0")
            }
            return Ok(Type::I32(n1 / n2))
        },
        (Type::F32(n1), Type::F32(n2)) => {
            if *n2 == 0.0 {
                return gerr!("Error: Cannot divide by 0")
            }
            return Ok(Type::F32(n1 / n2))
        }
        _ => gerr!("Error: Tyring to div [{:?}] and [{:?}]", a,b),
    }
}

#[allow(dead_code)]
pub fn modu(a : Type, b : Type) -> Result<Type, ERROR> {

    match (&a, &b) {
        (Type::I32(n1), Type::I32(n2)) => return Ok(Type::I32(n1 % n2)),
        (Type::F32(n1), Type::F32(n2)) => return Ok(Type::F32(n1 % n2)),
        _ => gerr!("Error: Tyring to run [mod] on [{:?}] and [{:?}]", a,b)
    }
}

#[allow(dead_code)]
pub fn pow(a : Type, b : Type) -> Result<Type, ERROR> {

    match (&a, &b) {
        (Type::I32(n1), Type::I32(n2)) => {
            if *n2 < 0 { 
                return gerr!("Error: Invaild exponent: [{:?}]", n2);
            }
            return Ok(Type::I32(n1.pow(*n2 as u32)))
        },
        (Type::F32(n1), Type::I32(n2)) => {
            if *n2 < 0 { 
                return gerr!("Error: Invaild exponent: [{:?}]", n2);
            }
            return Ok(Type::F32(n1.powi(*n2)))
        },
        (Type::F32(n1), Type::F32(n2)) => {
            if *n2 < 0.0 { 
                return gerr!("Error: Invaild exponent: [{:?}]", n2);
            }
            return Ok(Type::F32(n1.powf(*n2)))
        },
        _ => gerr!("Error: Tyring to pow [{:?}] and [{:?}]", a,b),
    }
}

#[allow(dead_code)]
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


#[allow(dead_code)]
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

#[allow(dead_code)]
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

pub fn smaller_or_eql(a : Type, b : Type) -> Result<Type, ERROR> {

    let (Type::BOOL(result1), Type::BOOL(result2)) = 
        (smaller(a.clone(), b.clone())?, eql(a.clone(), b.clone())?)
        else {
            return gerr!("Error: wrong args in [smaller_or_eql]: [{:?}, {:?}]", a,b);

    };

    Ok(Type::BOOL(result1 || result2))
}


pub fn bigger_or_eql(a : Type, b : Type) -> Result<Type, ERROR> {

    let (Type::BOOL(result1), Type::BOOL(result2)) = 
        (bigger(a.clone(), b.clone())?, eql(a.clone(), b.clone())?)
        else {
            return gerr!("Error: wrong args in [smaller_or_eql]: [{:?}, {:?}]", a,b);

    };

    Ok(Type::BOOL(result1 || result2))
}

#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn and(a : Type, b : Type) -> Result<Type, ERROR> {
    let (Type::BOOL(aa), Type::BOOL(bb)) = (&a,&b) else {
        return gerr!("Error: Unsuitable types for [and] [{:?} {:?}]", a, b);
    };

    Ok(Type::BOOL(*aa && *bb))
}


#[allow(dead_code)]
pub fn or(a : Type, b : Type) -> Result<Type, ERROR> {
    let (Type::BOOL(aa), Type::BOOL(bb)) = (&a,&b) else {
        return gerr!("Error: Unsuitable types for [or] [{:?} {:?}]", a, b);
    };

    Ok(Type::BOOL(*aa || *bb))
}
