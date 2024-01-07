use crate::{structs::{Type, ERROR, Globals, GError}, sgerr, util::{get_variable, split_qts, smart_split}, gerr};


pub fn stolist(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {

    let s = get_variable(&args[0], &glb.stack)?;

    let s = match s {
        Type::STR(s) => s,
        Type::CHAR(c) => c.to_string(),
        _ => return gerr!("Error: invalid argument given to [stolist]: [{:?}]", s)
    };

    let list : Vec<Type> = s
        .chars()
        .map(|c| Type::CHAR(c))
        .collect();

    Ok(Type::LIST(list))
}

pub fn ltostr(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let arg = get_variable(&args[0], &glb.stack)?;
    
    sgerr!(
        Type::LIST(list),
        arg,
        "Error: invalid argument given to [ltostr]: [{:?}]", arg
    );

    let s = {
        let mut s = "".to_string();
        for e in list {
            let Type::CHAR(c) = e else {
                return gerr!("Error: non char element in list given to [ltostr]: [{:?}]", e);
            };

            s.push(c);
        }
        s
    };


    Ok(Type::STR(s))
}

pub fn lines(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let arg = get_variable(&args[0], &glb.stack)?;

    sgerr!(
        Type::STR(s),
        arg,
        "Error: invalid argument given to [lines]: [{:?}]", arg
    );

    let l = {
        let mut v = vec![];
        for line in s.lines() {
            v.push(Type::STR(line.to_string()))
        }

        v
    };

    Ok(Type::LIST(l))
}

pub fn words(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {

    let arg = get_variable(&args[0], &glb.stack)?;

    sgerr!(
        Type::STR(s),
        arg,
        "Error: invalid argument given to [lines]: [{:?}]", arg
    );

    let l = {
        let mut v = vec![];
        for line in split_qts(&s) {
            v.push(Type::STR(line.to_string()))
        }

        v
    };

    Ok(Type::LIST(l))
}

pub fn s_words(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {

    let arg = get_variable(&args[0], &glb.stack)?;

    sgerr!(
        Type::STR(s),
        arg,
        "Error: invalid argument given to [lines]: [{:?}]", arg
    );

    let l : Vec<Type> = smart_split(&s)?.iter().map(|e| Type::STR(e.clone())).collect();

    Ok(Type::LIST(l))
}

pub fn trim(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {

    let arg = get_variable(&args[0], &glb.stack)?;

    sgerr!(
        Type::STR(s),
        arg,
        "Error: invalid argument given to [lines]: [{:?}]", arg
    );


    Ok(Type::STR(s.trim().to_string()))
}


